use super::*;

mod download;
mod upload;

/// Create a download task, note that the execute command needs to be [`DownloadTask::activated`]
///
/// # Arguments
///
/// * `remote_path`:
///
/// returns: Result<DownloadTask, QError>
///
/// # Examples
///
/// ```
/// use swarm_ssh::SwarmSSH;
/// ```
pub struct ContentResolver {
    content: Vec<u8>,
}


/// Create a download task, note that the execute command needs to be [`DownloadTask::activated`]
///
/// # Arguments
///
/// * `remote_path`:
///
/// returns: Result<DownloadTask, QError>
///
/// # Examples
///
/// ```
/// use swarm_ssh::SwarmSSH;
/// ```
pub struct UploadTask<'s> {
    content: Vec<u8>,
    target: PathBuf,
    permission: i32,
    session: &'s SSH2Session,
}


/// Create a download task, note that the execute command needs to be [`DownloadTask::activated`]
///
/// # Arguments
///
/// * `remote_path`:
///
/// returns: Result<DownloadTask, QError>
///
/// # Examples
///
/// ```
/// use swarm_ssh::SwarmSSH;
/// ```
pub struct DownloadTask<'s> {
    target: PathBuf,
    session: &'s SSH2Session,
}
