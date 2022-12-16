use super::*;

impl SwarmSSH {
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
    pub fn shell_runner(&self) -> QResult<ShellRunner> {
        Ok(ShellRunner { session: &self.session })
    }
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
pub struct ShellRunner<'s> {
    session: &'s SSH2Session,
}

impl ShellRunner<'_> {
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
    pub async fn execute(&self, command: &str) -> QResult<String> {
        let mut shell = self.session.channel_session()?;
        shell.exec(command)?;
        let mut stdout = String::new();
        shell.read_to_string(&mut stdout)?;
        shell.wait_close()?;
        Ok(stdout)
    }
}

