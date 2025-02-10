use std::{collections::HashMap, sync::Arc};

use crate::RdbArgs;


pub struct RedisStorage<'a> {
    pub db: HashMap<String, RedisValue>,
    pub rdbArgs: &'a Arc<RdbArgs>
    // mutex: 
}

#[derive(Debug)]
pub struct RedisValue {
    pub value: String,
    pub expiry: u128
}