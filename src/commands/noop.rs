use crate::commands::{Command, Result};
use memcached::Client;

pub struct NoOp {}

impl Command for NoOp {
    fn execute(&mut self, _client: &mut Client) -> Result {
        Ok(String::from("Command not found"))
    }
}
