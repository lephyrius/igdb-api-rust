use std::any::type_name;
use crate::apicalypse_builder::ApicalypseBuilder;
use crate::client::IGDBApiError::AuthError;
use microjson::JSONParsingError;
use prost::DecodeError;
use thiserror::Error;
use crate::igdb::Count;

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

pub struct Client {
    client_id: String,
    client_secret: String,
    client: reqwest::Client,
    client_access_token: String,
}

impl Client {
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        Client {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            client: reqwest::Client::new(),
            client_access_token: String::default(),
        }
    }

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

    pub async fn request_raw<M: prost::Message + Default>(
        &mut self,
        query: &str,
    ) -> Result<M, IGDBApiError> {
        self.request_api(query, format!("https://api.igdb.com/v4/{}", endpoint_name::<M>())).await
    }


    pub async fn request_count<M: prost::Message + Default>(
        &mut self,
        query: &'static ApicalypseBuilder,
    ) -> Result<Count, IGDBApiError> {
        let query_string = query.to_query();
        self.request_count_raw::<M>(query_string.as_str()).await
    }

    pub async fn request_count_raw<M: prost::Message + Default>(
        &mut self,
        query: &str,
    ) -> Result<Count, IGDBApiError> {
        self.request_api(query, format!("https://api.igdb.com/v4/{}/count", endpoint_name::<M>())).await
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
            .send()
            .await?
            .bytes()
            .await?;
        M::decode(bytes).map_err(Into::into)
    }
}

impl Default for Client {
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
    use crate::igdb::{AlternativeName, Game, GameEngineLogoResult, ThemeResult};
    use super::*;

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
