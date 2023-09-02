use crate::commands::{Command, Result, CommandError};
use memcached::Client;
use memcached::proto::{Operation, Error};

pub struct Get {
    key: String
}

pub struct GetBuilder {
    key: String
}

impl GetBuilder {
    pub fn new() -> GetBuilder {
        GetBuilder { key: "".to_string() }
    }
    pub fn key(mut self, key: String) -> GetBuilder {
        self.key = key;
        self
    }

    pub fn build(self) -> Get {
        assert!(self.key.len() > 0);
        Get {
            key: self.key
        }
    }
}

impl Command for Get {
    fn execute(&mut self, client: &mut Client) -> Result {
        let val = client.get(self.key.as_bytes());
        match val {
            Ok((s, _)) => {
                Ok(String::from_utf8(s).unwrap())
            },
            Err(err) => {
                match err {
                    Error::BinaryProtoError(bin_err) => {
                        match bin_err.status() {
                            memcached::proto::binary::Status::KeyNotFound => {
                                Ok(String::from("NOT FOUND"))
                            },
                            _ => {
                                Err(CommandError{
                                    message: bin_err.to_string()
                                })
                            }
                        }
                        
                    },
                    _ => {
                        Err(CommandError{
                            message: err.to_string()
                        })
                    }
                }
            }
        }
    }
}