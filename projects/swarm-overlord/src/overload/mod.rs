use std::{
    io::{Read, Write},
    net::ToSocketAddrs,
};

use diagnostic_quick::error_3rd::SSH2Session;

pub struct SwarmOverload {
    session: SSH2Session,
}

pub mod github;