use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use crate::Error;

pub trait OpenOptionExt {
    fn open_read(path: &Path) -> Result<File, Error>;
    fn open_append(path: &Path) -> Result<File, Error>;
}

impl OpenOptionExt for OpenOptions {
    fn open_read(path: &Path) -> Result<File, Error> {
        OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|_| Error::FileOpenError)
    }

    fn open_append(path: &Path) -> Result<File, Error> {
        OpenOptions::new()
            .append(true)
            .write(true)
            .open(&path)
            .map_err(|_| Error::FileOpenError)
    }
}
