Swarm SSH
=========

Manage remote computers via ssh protocol.

## Login

```rust
use diagnostic_quick::QResult;
use swarm_ssh::SwarmSSH;

async fn test_password() -> QResult<SwarmSSH> {
    SwarmSSH::login_password("192.168.1.100:22", "root", "password").await
}
```

## Upload

```rust
use std::path::PathBuf;
use diagnostic_quick::QResult;
use swarm_ssh::SwarmSSH;

async fn test_password() -> QResult {
    let ssh = SwarmSSH::login_password("192.168.1.100:22", "root", "password").await?;
    let path = PathBuf::from("Cargo.toml");
    ssh.upload_task(&path, "/tmp/Cargo.toml")?.execute().await?;
    Ok(())
}
```

## Download

```rust
use diagnostic_quick::QResult;
use swarm_ssh::SwarmSSH;

async fn test_password() -> QResult<SwarmSSH> {
    SwarmSSH::login_password("192.168.1.100:22", "root", "password").await
}
```