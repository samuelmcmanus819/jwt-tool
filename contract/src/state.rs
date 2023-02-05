use core::fmt;

use schemars::JsonSchema;
use secret_toolkit::{
  storage::{
    Item 
  }
};
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
  Storage, Timestamp, 
};
use cosmwasm_storage::{
  singleton, 
  singleton_read, 
  ReadonlySingleton, 
  Singleton
};

pub const PRIVKEY_KEY: &[u8] = b"privkey";


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Header {
  pub typ: String,
  pub alg: String
}

impl fmt::Display for Header {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{{ typ: {}, alg: {} }}", self.typ, self.alg)
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Payload {
  pub address: String,
  pub exp: Timestamp
}

impl fmt::Display for Payload {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{{ address: {}, exp: {} }}", self.address, self.exp)
  }
}
pub static PRIVKEY: Item<[u8; 32]> = Item::new(PRIVKEY_KEY);

pub fn privkey(storage: &mut dyn Storage) -> Singleton<[u8; 32]> {
  singleton(storage, PRIVKEY_KEY)
}

pub fn privkey_read(storage: &dyn Storage) -> ReadonlySingleton<[u8; 32]> {
  singleton_read(storage, PRIVKEY_KEY)
}
