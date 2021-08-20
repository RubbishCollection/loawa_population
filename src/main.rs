mod open_option_ext;
pub use open_option_ext::*;
mod error;
pub use error::*;

mod loawa_scraper;
pub use loawa_scraper::*;
mod loawa_connector;
pub use loawa_connector::*;
mod loawa_parser;
pub use loawa_parser::*;
mod uri_iterator;
pub use uri_iterator::*;

mod jobs;
pub use jobs::*;

mod data_writer;
pub use data_writer::*;
mod csv_rw;
pub use csv_rw::*;
mod csv_row;
pub use csv_row::*;

mod today;
pub use today::*;

use csv::{ReaderBuilder, WriterBuilder};
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let connector = LoawaConnector::new("loawa_population_statistics".to_string())?;
    let scraper = LoawaScraper::init(connector);
    let uri_iter = UriIterator::init('a', 'p');
    let mut data = scraper.get_data(uri_iter).await?;

    let section = [
        1340, 1355, 1370, 1385, 1400, 1415, 1430, 1445, 1460, 1475, 1490, 1505, 1520, 1535, 1550,
        1575,
    ];

    for i in 0..data.len() {
        let path = format!("./data_files/{}.csv", section[i].to_string());
        let path = Path::new(&path);

        let order_vec = {
            let file = OpenOptions::open_read(&path)?;
            let mut reader =
                CsvReader::new(ReaderBuilder::new().has_headers(false).from_reader(file));

            reader.get_order_vec()?
        };

        let file = OpenOptions::open_append(path)?;
        let writer = CsvWriter::new(WriterBuilder::new().has_headers(false).from_writer(file));

        let jobs = data.remove(0);
        let row = Row::new(get_today().unwrap(), jobs.to_sorted_vec(order_vec));

        writer.write(row)?;
    }

    println!("Done!");
    Ok(())
}
