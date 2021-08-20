use nipper::Document;

use crate::Error;
use crate::{Jobs, UriIterator};

use crate::parse_document;
use crate::LoawaConnector;

pub struct LoawaScraper {
    connector: LoawaConnector,
}

impl LoawaScraper {
    pub fn init(connector: LoawaConnector) -> Self {
        Self { connector }
    }

    pub async fn get_data(&self, uri_iter: UriIterator) -> Result<Vec<Jobs>, Error> {
        let mut data = Vec::new();

        for uri in uri_iter {
            let doc = self.connector.get_document(uri).await?;
            let jobs = parse_document(doc);
            data.push(jobs);
        }

        Ok(data)
    }
}
