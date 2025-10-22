use std::any::Any;
use std::collections::HashMap;
use std::convert::Infallible;

pub struct TwitchService;

impl TwitchService {
    pub fn send_raw_template(template: &str, values: HashMap<String, Box<dyn Any>>) -> Infallible {
        // all errors should just be logged
        todo!()
    }
}