use fs::File;
use std::{
    fs,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    path::{Path, PathBuf},
};

use diagnostic_quick::{error_3rd::SSH2Session, QError, QResult};

/// Connect to a remote computer via the ssh protocol.
pub struct SwarmSSH {
    session: SSH2Session,
}

pub mod scp;
pub mod shell;

impl SwarmSSH {
    /// Log in to the remote computer with username and password
    ///
    /// # Arguments
    ///
    /// * `address`: The address of the remote computer, such as `192.168.1.100:22`
    /// * `user`: The username of the remote computer, such as `root`
    /// * `password`: The password of the remote computer, such as `password`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swarm_ssh::SwarmSSH;
    /// let ssh = SwarmSSH::login_password("192.168.1.100:22", "root", "password").await?;
    /// ```
    pub async fn login_password<A>(address: A, user: &str, password: &str) -> QResult<Self>
        where
            A: ToSocketAddrs,
    {
        let tcp = TcpStream::connect(address)?;
        let mut session = SSH2Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        session.userauth_password(user, password)?;
        if !session.authenticated() {
            Err("Authentication failed")?
        }
        Ok(Self { session })
    }
}

