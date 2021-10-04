use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryParams<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryParams<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryParams<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let (key, value) = if let Some(i) = sub_str.find('=') {
                (&sub_str[..i], &sub_str[i + 1..])
            } else {
                (&sub_str[..], "")
            };

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_value) => {
                        *existing = Value::Multiple(vec![prev_value, value]);
                    }
                    Value::Multiple(vec) => vec.push(value),
                })
                .or_insert(Value::Single(value));
        }

        let query_params = QueryParams { data };
        return query_params;
    }
}
