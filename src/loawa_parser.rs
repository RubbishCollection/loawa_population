use nipper::Document;

use crate::Jobs;

pub fn parse_document(doc: Document) -> Jobs {
    let jobs = doc.select(".col-6.pl-1.pr-2");
    let names = jobs.select(".w700.pt-1:not(.pl-1)").iter().skip(1);
    let counts = jobs.select(".pl-1.pt-1.w700").iter();

    let data = names
        .zip(counts)
        .map(|(name, count)| {
            (
                name.text().to_string(),
                count
                    .text()
                    .to_string()
                    .replace(",", "")
                    .replace("명", "")
                    .parse()
                    .unwrap(),
            )
        })
        .collect();
    Jobs::new(data)
}

#[cfg(test)]
mod test {
    use crate::parse_document;

    #[test]
    fn parsing_test() {
        let html = include_str!("../test_files/loawa_test.htm");
        let doc = nipper::Document::from(html);
        let jobs = parse_document(doc);
        let data = jobs.data();

        assert_eq!(data.get("바드"), Some(&91993u32));
        assert_eq!(data.get("데빌헌터"), Some(&10446u32));
    }
}
