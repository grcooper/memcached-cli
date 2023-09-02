use crate::commands::{Command, Result, CommandError};
use memcached::Client;
use memcached::proto::Operation;
pub struct Set {
    key: String,
    value: String,
    flags: u32,
    expiration: u32,
}

#[derive(Default)]
pub struct SetBuilder {
    key: String,
    value: String,
    flags: u32,
    expiration: u32,
}

impl SetBuilder {
    pub fn new() -> SetBuilder {
        SetBuilder {
            key: "".to_string(),
            value: "".to_string(),
            expiration: 0,
            flags: 0,
        }
    }
    pub fn key(mut self, key: String) -> SetBuilder {
        self.key = key;
        self
    }
    pub fn value(mut self, value: String) -> SetBuilder {
        self.value = value;
        self
    }
    pub fn flags(mut self, flags: u32) -> SetBuilder {
        self.flags = flags;
        self
    }
    pub fn expiration(mut self, expiration: u32) -> SetBuilder {
        self.expiration = expiration;
        self
    }
    pub fn build(self) -> Set {
        assert!(self.key.len() > 0);
        assert!(self.key.len() > 0);
        Set {
            key: self.key,
            value: self.value,
            expiration: self.expiration,
            flags: self.flags,
        }
    }
}

impl Command for Set {
    fn execute(&mut self, client: &mut Client) -> Result {
        let val = client.set(self.key.as_bytes(), self.value.as_bytes(), self.flags, self.expiration);
        match val {
            Ok(_) => {
                Ok(String::from("Value set"))
            },
            Err(e) => {
                Err(CommandError {
                    message: e.to_string()
                })
            }
        }
    }
}