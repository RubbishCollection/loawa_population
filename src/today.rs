use chrono::prelude::*;

pub fn get_today() -> Option<String> {
    Some(Local::today().to_string().get(0..10)?.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn today_test() {
        println!("{}", get_today().unwrap());
    }
}
