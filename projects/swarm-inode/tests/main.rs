use diagnostic_quick::QResult;

use swarm_ssh::SwarmSSH;

#[test]
fn ready() {
    println!("it works!")
}

#[ignore]
#[tokio::test]
async fn test_ability() -> QResult {
    let client = SwarmSSH::login_password("192.168.1.100:22", "root", "projecta").await?;

    let shell = client.shell_runner()?;
    println!("{}", shell.execute("ll").await?);

    // 上传文件
    let data: &[u8] = include_bytes!("../Cargo.toml");
    client.upload_task(data, "/tmp/Cargo.toml")?.with_permission(0o755).execute().await?;
    let download = client.download_task("/tmp/Cargo.toml")?.execute().await?;
    println!("{}", String::from_utf8_lossy(&download));
    Ok(())
}
