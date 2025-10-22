use std::any::Any;
use std::collections::HashMap;

pub struct TwitchService;

impl TwitchService {
    pub fn send_raw_template(&self, _template: &str, _values: HashMap<String, Box<dyn Any>>) {
        // all errors should just be logged
        todo!()
    }
}