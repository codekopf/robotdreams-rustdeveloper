use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    sender: String,
    content: String,
}

impl Message {

    pub fn new(sender: &str, content: &str) -> Self {
        Message {
            sender: sender.to_string(),
            content: content.to_string(),
        }
    }

}