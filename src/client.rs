use std::any::type_name;
use crate::apicalypse_builder::ApicalypseBuilder;
use crate::client::IGDBApiError::AuthError;
use microjson::JSONParsingError;
use prost::DecodeError;
use thiserror::Error;
use crate::igdb::Count;

const LIB_VERSION_HEADER: &str = concat!("igdb-api-rust v " ,env!("CARGO_PKG_VERSION"));

#[derive(Error, Debug)]
pub enum IGDBApiError {
    #[error("Something is wrong with the auth please check the credentials: {0:?}")]
    AuthError(JSONParsingError),
    #[error("Cannot decode API response: {0:?}")]
    ApiResponseDecodeError(#[from] DecodeError),
    #[error("Cannot request server")]
    Request(#[from] reqwest::Error),
    #[error("unknown API error")]
    Unknown,
}

impl From<JSONParsingError> for IGDBApiError {
    fn from(value: JSONParsingError) -> Self {
        AuthError(value)
    }
}

/// The IGDB API client.
pub struct Client {
    client_id: String,
    client_secret: String,
    client: reqwest::Client,
    client_access_token: String,
    endpoint: String,
}

impl Client {
    /// Create a new client.
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        Client {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            client: reqwest::Client::new(),
            client_access_token: String::default(),
            endpoint: "https://api.igdb.com/v4".to_string()
        }
    }
    /// Set a custom endpoint for use with the CORS proxy.
    /// ```
    /// use igdb_api_rust::client::Client;
    /// let mut client = Client::new("test","test").with_endpoint("https://example.com/v4");
    /// ```
    pub fn with_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = endpoint.to_string();
        self
    }

    /// Request the IGDB API for a protobuf response.
    pub async fn request<M: prost::Message + Default>(
        &mut self,
        query: &'static ApicalypseBuilder,
    ) -> Result<M, IGDBApiError> {
        let query_string = query.to_query();
        self.request_raw(query_string.as_str()).await
    }


    async fn check_access_token(&mut self) -> Result<(), IGDBApiError> {
        if self.client_access_token.is_empty() {
            use microjson::JSONValue;
            let resp = self
                .client
                .post("https://id.twitch.tv/oauth2/token")
                .query(&[
                    ("client_id", self.client_id.as_str()),
                    ("client_secret", self.client_secret.as_str()),
                    ("grant_type", "client_credentials"),
                ])
                .send()
                .await
                .map(|response| response.text())?;

            self.client_access_token = JSONValue::parse(resp.await?.as_str())?
                .get_key_value("access_token")?
                .read_string()?
                .to_string();
        }
        Ok(())
    }

    /// Request the IGDB API for a protobuf response.
    /// This is the raw version of the request method.
    /// It allows you to pass a query string directly.
    /// ```
    /// use igdb_api_rust::client::Client;
    /// let mut client = Client::new("test","test");
    /// let query = "fields name; limit 5;";
    /// let response = client.request_raw::<igdb_api_rust::igdb::Game>(query);
    /// ```
    pub async fn request_raw<M: prost::Message + Default>(
        &mut self,
        query: &str,
    ) -> Result<M, IGDBApiError> {
        self.request_api(query,  endpoint_name::<M>()).await
    }

    /// Request the IGDB API for a protobuf response for the count endpoint.
    /// ```
    /// use igdb_api_rust::apicalypse_builder::ApicalypseBuilder;
    /// use igdb_api_rust::client::Client;
    /// let mut client = Client::new("test","test");
    /// let query = ApicalypseBuilder::default().filter("id > 1337");
    /// let response = client.request_count::<igdb_api_rust::igdb::Game>(query);
    /// ```
    pub async fn request_count<M: prost::Message + Default>(
        &mut self,
        query: &'static ApicalypseBuilder,
    ) -> Result<Count, IGDBApiError> {
        let query_string = query.to_query();
        self.request_count_raw::<M>(query_string.as_str()).await
    }

    /// Request the IGDB API for a protobuf response for the count endpoint.
    /// This is the raw version of the request_count method.
    /// It allows you to pass a query string directly.
    /// ```
    /// use igdb_api_rust::client::Client;
    /// let mut client = Client::new("test","test");
    /// let query = "w id > 1337";
    /// let response = client.request_count_raw::<igdb_api_rust::igdb::Game>(query);
    /// ```
    pub async fn request_count_raw<M: prost::Message + Default>(
        &mut self,
        query: &str,
    ) -> Result<Count, IGDBApiError> {
        self.request_api(query, format!("{}/count", self.endpoint_url::<M>())).await
    }


    fn endpoint_url<M: prost::Message + Default>(&self) -> String {
        format!("{}/{}", self.endpoint, endpoint_name::<M>())
    }

    async fn request_api<M: prost::Message + Default>(&mut self, query: &str, url: String) -> Result<M, IGDBApiError> {
        if let Err(error) = self.check_access_token().await {
            return Err(error);
        }
        let bytes = self
            .client
            .post(url)
            .body(query.to_string())
            .bearer_auth(&self.client_access_token)
            .header("client-id", &self.client_id)
            .header("x-user-agent", LIB_VERSION_HEADER )
            .send()
            .await?
            .bytes()
            .await?;
        M::decode(bytes).map_err(Into::into)
    }
}

impl Default for Client {
    /// Get a client with the credentials from the environment variables.
    fn default() -> Self {
        use std::env::var;
        Self::new(
            &var("IGDB_API_ID").expect("for IGDB_API_ID env var to be defined"),
            &var("IGDB_API_SECRET").expect("for IGDB_API_SECRET env var to be defined"),
        )
    }
}


fn endpoint_name<M: prost::Message + Default>() -> String {
    let message_name = type_name::<M>().split("::").last().unwrap_or_default();
    if message_name == "Person" {
        "people".to_string()
    } else {
        use heck::AsSnekCase;
        AsSnekCase(message_name).to_string().replace("_result", "") + "s"
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use crate::igdb::{AlternativeName, Game, GameEngineLogoResult, ThemeResult};
    use super::*;

    #[test]
    fn test_default() {
        // Set the environment variables that the default method expects to read
        env::set_var("IGDB_API_ID", "test_id_env");
        env::set_var("IGDB_API_SECRET", "test_secret_env");

        // Call the default method
        let client = Client::default();

        assert_eq!(client.client_id, "test_id_env");
        assert_eq!(client.client_secret, "test_secret_env");

        // Clean up by removing the environment variables if needed
        env::remove_var("IGDB_API_ID");
        env::remove_var("IGDB_API_SECRET");
    }

    #[test]
    fn test_new() {
        let client = Client::new("test_id", "test_secret");

        // Basic checks to make sure the client was constructed correctly
        assert_eq!(client.client_id, "test_id");
        assert_eq!(client.client_secret, "test_secret");
        assert_eq!(client.client_access_token, "");
    }


    #[test]
    fn endpoint_name_games() {
        assert_eq!(
            "games",
            endpoint_name::<Game>()
        );
    }


    #[test]
    fn endpoint_name_alternative_names() {
        assert_eq!(
            "alternative_names",
            endpoint_name::<AlternativeName>()
        );
    }

    #[test]
    fn endpoint_name_game_engine_logos() {
        assert_eq!(
            "game_engine_logos",
            endpoint_name::<GameEngineLogoResult>()
        );
    }


    #[test]
    fn endpoint_name_themes() {
        assert_eq!(
            "themes",
            endpoint_name::<ThemeResult>()
        );
    }


}
