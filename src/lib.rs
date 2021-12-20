use regex::{Regex, Error};
use std::marker::PhantomData;
use cached::proc_macro::cached;

#[cached(size=1)]
fn get_regex(s: String) -> Result<Regex, Error> {
    Regex::new(&s)
}

impl<'a, T: From<&'a str>> ReString<'a, T> {
    pub fn new(s: &'a str, sre: &str) -> Option<Self> {
        let re = match get_regex(String::from(sre)) {
            Ok(c) => c,
            Err(_) => return None,
        };
        
        let m = re.find(&s)?;

        if m.as_str().is_empty() {
            return None;
        } else {
            let result = T::from(m.as_str());
            let index = m.end();
            
            Some(Self {result, index, phantom_data: PhantomData})
        }
    }
}

pub struct ReString<'a, T: From<&'a str>> {
    pub result : T,
    pub index : usize,
    phantom_data: PhantomData<&'a T>,
}


