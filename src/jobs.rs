use std::collections::HashMap;

pub struct Jobs {
    data: HashMap<String, u32>,
}

impl Jobs {
    pub fn new(data: HashMap<String, u32>) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &HashMap<String, u32> {
        &self.data
    }

    pub fn to_sorted_vec(self, class_order: Vec<String>) -> Vec<u32> {
        let mut result = Vec::new();
        for class_name in class_order {
            match self.data.get(&class_name) {
                Some(v) => result.push(*v),
                None => result.push(0),
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::Jobs;

    #[test]
    fn jobs_test() {
        let mut data = HashMap::new();
        data.insert("바드".to_string(), 1111);
        data.insert("모닥불".to_string(), 3333);
        data.insert("배마".to_string(), 5555);
        data.insert("데빌헌터".to_string(), 9999);

        let order = vec![
            "바드".to_string(),
            "데빌헌터".to_string(),
            "배마".to_string(),
            "모닥불".to_string(),
        ];

        let jobs = Jobs::new(data);
        let data_vec = jobs.to_sorted_vec(order);

        assert_eq!(data_vec[0], 1111);
        assert_eq!(data_vec[1], 9999);
        assert_eq!(data_vec[2], 5555);
        assert_eq!(data_vec[3], 3333);
    }
}
