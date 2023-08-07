#[derive(Default)]
pub struct ApicalypseBuilder {
    filter: String,
    limit: usize,
    offset: usize,
    fields: String,
    exclude: String,
    sort: String,
}

impl ApicalypseBuilder {
    pub fn filter(&mut self, filter: &str) -> &mut Self {
        self.filter = filter.to_string();
        self
    }

    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = limit;
        self
    }

    pub fn offset(&mut self, offset: usize) -> &mut Self {
        self.offset = offset;
        self
    }

    pub fn fields(&mut self, fields: &str) -> &mut Self {
        self.fields = fields.to_string();
        self
    }

    pub fn exclude(&mut self, exclude: &str) -> &mut Self {
        self.exclude = exclude.to_string();
        self
    }

    pub fn sort(&mut self, sort: &str) -> &mut Self {
        self.sort = sort.to_string();
        self
    }

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
