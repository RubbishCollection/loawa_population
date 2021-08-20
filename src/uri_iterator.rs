pub struct UriIterator {
    base: String,
    current_section: char,
    last_section: char,
}

impl UriIterator {
    pub fn init(first_section: char, last_section: char) -> Self {
        Self {
            base: "https://loawa.com/stat_process.php?search=".to_string(),
            current_section: first_section,
            last_section,
        }
    }
}

impl Iterator for UriIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_section <= self.last_section {
            let uri = format!("{}{}", self.base, self.current_section);
            self.current_section = char::from_u32(self.current_section as u32 + 1)?;
            Some(uri)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn iterator_test() {
        let mut uris = UriIterator::init('a', 'c');

        assert_eq!(
            "https://loawa.com/stat_process.php?search=a",
            &uris.next().unwrap()
        );
        assert_eq!(
            "https://loawa.com/stat_process.php?search=b",
            &uris.next().unwrap()
        );
        assert_eq!(
            "https://loawa.com/stat_process.php?search=c",
            &uris.next().unwrap()
        );
        assert_eq!(None, uris.next());
    }
}
