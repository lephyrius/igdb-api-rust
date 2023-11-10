#[derive(Default, Clone)]
pub struct ApicalypseBuilder {
    filter: String,
    limit: usize,
    offset: usize,
    fields: String,
    exclude: String,
    sort: String,
}

/// Builder for Apicalypse queries
impl ApicalypseBuilder {

    /// Add a filter to the query.
    pub fn filter(mut self, filter: &str) -> Self {
        self.filter = filter.to_string();
        self
    }

    /// Add a limit of entries to the query.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Add an offset from where the entries should start in the results to the query.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Specify fields to be returned by the query.
    pub fn fields(mut self, fields: &str) -> Self {
        self.fields = fields.to_string();
        self
    }

    /// Exclude fields from the query.
    pub fn exclude(mut self, exclude: &str) -> Self {
        self.exclude = exclude.to_string();
        self
    }

    /// Order on some specific fields.
    pub fn sort(mut self, sort: &str) -> Self {
        self.sort = sort.to_string();
        self
    }

    /// Build the query string.
    pub fn to_query(&self) -> String {
        format!(
            "{}{}{}{}{}{}",
            wrap_statement("f", &self.fields),
            wrap_statement("x", &self.exclude),
            wrap_statement("w", &self.filter),
            wrap_statement("l", &self.limit),
            wrap_statement("o", &self.offset),
            wrap_statement("s", &self.sort),
        )
    }
}

fn wrap_statement<T: ToString>(prefix: &str, statemet: &T) -> String {
    let statement_string = statemet.to_string();
    if statement_string.is_empty() || statement_string == "0" {
        String::default()
    } else {
        format!("{} {};", prefix, statement_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter() {
        assert_eq!(
            "w id = 1337;",
            ApicalypseBuilder::default().filter("id = 1337").to_query()
        );
    }

    #[test]
    fn limit() {
        assert_eq!(
            "l 1337;",
            ApicalypseBuilder::default().limit(1337).to_query()
        );
    }

    #[test]
    fn offset() {
        assert_eq!(
            "o 1337;",
            ApicalypseBuilder::default().offset(1337).to_query()
        );
    }

    #[test]
    fn fields() {
        assert_eq!(
            "f id,name;",
            ApicalypseBuilder::default().fields("id,name").to_query()
        );
    }

    #[test]
    fn exclude() {
        assert_eq!(
            "x id,name;",
            ApicalypseBuilder::default().exclude("id,name").to_query()
        );
    }

    #[test]
    fn sort() {
        assert_eq!(
            "s id desc;",
            ApicalypseBuilder::default().sort("id desc").to_query()
        );
    }

    #[test]
    fn all() {
        assert_eq!(
            "f *;x id,name;w id = 1337;l 55;o 66;s id desc;",
            ApicalypseBuilder::default()
                .filter("id = 1337")
                .limit(55)
                .offset(66)
                .fields("*")
                .exclude("id,name")
                .sort("id desc")
                .to_query()
        );
    }
}
