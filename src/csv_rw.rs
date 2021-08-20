use crate::{DataWriter, Error, FirstRow, Jobs, Row};

use csv::{Reader, Writer};
use std::io::{Read, Write};

use serde::Serialize;

pub struct CsvReader<R>
where
    R: Read,
{
    reader: Reader<R>,
}

impl<R> CsvReader<R>
where
    R: Read,
{
    pub fn new(reader: Reader<R>) -> Self {
        Self { reader }
    }

    pub fn get_order_vec(&mut self) -> Result<Vec<String>, Error> {
        let mut rows = self.reader.deserialize();
        let first: FirstRow = rows
            .next()
            .ok_or(Error::RowReadingError)?
            .map_err(|_| Error::RowReadingError)?;

        Ok(first.class_vec())
    }
}

pub struct CsvWriter<W>
where
    W: Write,
{
    writer: Writer<W>,
}

impl<W> CsvWriter<W>
where
    W: Write,
{
    pub fn new(writer: Writer<W>) -> Self {
        Self { writer }
    }
}

impl<W> DataWriter for CsvWriter<W>
where
    W: Write,
{
    type Item = Row;

    fn write(mut self, data: Self::Item) -> Result<(), Error> {
        self.writer
            .serialize(data)
            .map_err(|_| Error::RowWritingError)?;
        self.writer
            .into_inner()
            .map_err(|_| Error::RowWritingError)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use csv::{ReaderBuilder, WriterBuilder};
    use tempfile::NamedTempFile;

    use super::*;
    #[test]
    fn reader_test() {
        let reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path("./test_files/test.csv")
            .unwrap();
        let mut csv = CsvReader::new(reader);
        let order_vec = csv.get_order_vec().unwrap();
        assert_eq!(order_vec.first().unwrap(), "바드");
        assert_eq!(order_vec.last().unwrap(), "데빌헌터");
    }

    #[test]
    fn writer_test() {
        let mut data = Vec::new();
        data.push(111);
        data.push(222);
        data.push(333);
        data.push(444);
        data.push(555);
        let row = Row::new("2021-08-18".to_string(), data);

        let file1 = NamedTempFile::new().unwrap();
        let mut file2 = file1.reopen().unwrap();

        let writer = WriterBuilder::new().has_headers(false).from_writer(file1);
        let csv = CsvWriter::new(writer);

        csv.write(row).unwrap();

        let mut buf = String::new();
        file2.read_to_string(&mut buf).unwrap();
        assert_eq!("2021-08-18,111,222,333,444,555\n", buf);
    }
}
