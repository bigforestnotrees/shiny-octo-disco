use regex::{Regex, Error};
use std::marker::PhantomData;
use cached::proc_macro::cached;

#[cached(size=1)]
fn get_regex(s: String) -> Result<Regex, Error> {
    Regex::new(&s)
}

impl<'a, T: From<&'a str>> Iterator for ReString<'a, T> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.string.len() {
            return None;
        }
        self.string = &self.string[self.index..];

        let re = match get_regex(String::from(self.regex)) {
            Ok(c) => c,
            Err(_) => return None,
        };

        let m = re.find(self.string)?;

        if m.as_str().is_empty() {
            return None;
        } else {
            self.result = T::from(m.as_str());
            self.index = m.end();
            
            Some(String::from(self.string))
        }
    }
}

impl<'a, T: From<&'a str>> ReString<'a, T> {
    pub fn new(s: &'a str, sre: &'a str) -> Option<Self> {
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
            
            Some(Self {string: s, result, index, regex: sre, phantom_data: PhantomData})
        }
    }
}

pub struct ReString<'a, T: From<&'a str>> {
    pub string : &'a str,
    pub result : T,
    pub index : usize,
    pub regex : &'a str,
    phantom_data: PhantomData<&'a T>,
}


