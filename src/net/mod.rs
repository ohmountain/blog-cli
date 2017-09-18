extern crate curl;

use std::str;
use self::curl::easy::Easy;
use self::curl::easy::Form;

use std::collections::HashMap;

pub struct Post {
    pub url: String,
    pub parameters: HashMap<String, String>,
}


pub struct Get {
    pub url: String,
}

pub trait Perform {
    fn perform(&mut self) -> String;
}


impl Perform for Post {
    fn perform(&mut self) -> String {
        let mut data = Vec::new();
        let mut easy = Easy::new();

        easy.url(&self.url.as_str()).unwrap();
        easy.post(true).unwrap();

        let mut form = Form::new();

        for (k, v) in &self.parameters {
             form.part(k.as_str()).contents(v.as_bytes()).add().unwrap();
        }

        easy.httppost(form).unwrap();

        {
            let mut transfer = easy.transfer();

            transfer.write_function(|raw| {
                data.extend_from_slice(raw);
                Ok(raw.len())
            }).unwrap();

            transfer.perform().unwrap();
        }

        String::from(str::from_utf8(data.as_slice()).unwrap_or(""))
    }
}


impl Perform for Get {

    fn perform(&mut self) -> String {
        let mut data = Vec::new();
        let mut easy = Easy::new();

        easy.url(&self.url.as_str()).unwrap();
        easy.get(true).unwrap();

        {
            let mut transfer = easy.transfer();

            transfer.write_function(|raw| {
                data.extend_from_slice(raw);
                Ok(raw.len())
            }).unwrap();

            transfer.perform().unwrap();
        }

        String::from(str::from_utf8(data.as_slice()).unwrap_or(""))
    }
}
