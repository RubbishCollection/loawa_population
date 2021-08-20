use crate::{Error, Jobs};
pub trait DataWriter {
    type Item;
    fn write(self, data: Self::Item) -> Result<(), Error>;
}
