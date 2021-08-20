use thiserror::Error;

use csv::IntoInnerError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("[reqwest] Fail to build http client")]
    HttpClientBuildingError(#[from] reqwest::Error),
    #[error("[csv] Fail to read row")]
    RowReadingError,
    #[error("[csv] Fail to write row")]
    RowWritingError,
    #[error("[std::io] Fail to open file")]
    FileOpenError,
}
