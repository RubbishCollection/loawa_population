use nipper::Document;
use reqwest::cookie::Jar;
use reqwest::Client as HttpClient;

use crate::Error;
pub struct LoawaConnector {
    client: HttpClient,
}

impl LoawaConnector {
    pub fn new(user_agent: String) -> Result<Self, Error> {
        let jar = std::sync::Arc::new(Jar::default());
        let client = HttpClient::builder()
            .cookie_store(true)
            .cookie_provider(jar)
            .user_agent(user_agent)
            .build()?;

        Ok(Self { client })
    }

    pub async fn get_document(&self, uri: String) -> Result<Document, Error> {
        let html = self.client.get(uri).send().await?.text().await?;
        Ok(Document::from(&html))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn connector_test() {
        let connector = LoawaConnector::new("loawa_population_statistic".to_string()).unwrap();
        let doc = connector
            .get_document("https://loawa.com/stat_process.php?search=a".to_string())
            .await
            .unwrap();
    }
}
