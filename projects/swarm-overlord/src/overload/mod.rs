use std::{
    io::{Read, Write},
    net::ToSocketAddrs,
};

use sled_typed::Database;

pub mod github;

pub struct SwarmOverload {
    database: Database,
    base_url: String,
    base_path: String,
}
