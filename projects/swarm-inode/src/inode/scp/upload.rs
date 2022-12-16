use super::*;

impl SwarmSSH {
    /// Create a upload task, note that the execute command needs to be [`DownloadTask::activated`]
    ///
    /// # Arguments
    ///
    /// * `content`:
    /// * `remote_path`:
    ///
    /// returns: Result<UploadTask, QError>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use diagnostic_quick::QResult;
    /// # use swarm_ssh::SwarmSSH;
    /// async fn test_upload() -> QResult {
    ///     let ssh = SwarmSSH::login_password("192.168.1.100:22", "root", "password").await?;
    ///     let data: &[u8] = include_bytes!("../inode");
    ///     ssh.upload_task(data, "/tmp/inode")?.with_permission(0o644).execute().await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn upload_task<C, P>(&self, content: C, remote_path: P) -> QResult<UploadTask>
    where
        P: AsRef<Path>,
        ContentResolver: TryFrom<C, Error = QError>,
    {
        let content = ContentResolver::try_from(content)?.content;
        Ok(UploadTask { content, target: remote_path.as_ref().to_path_buf(), permission: 0o644, session: &self.session })
    }
}

impl<'s> UploadTask<'s> {
    /// Get current permission setting
    pub fn get_permission(&self) -> i32 {
        self.permission
    }
    /// Set permissions after file upload
    pub fn set_permission(&mut self, permission: i32) {
        self.permission = permission;
    }
    /// Set permissions after file upload
    pub fn with_permission(mut self, permission: i32) -> Self {
        self.permission = permission;
        self
    }
    /// Get current target path
    pub fn get_remote_path(&self) -> &Path {
        &self.target
    }
    /// Set target path after file upload
    pub fn set_remote_path<P: AsRef<Path>>(&mut self, remote_path: P) {
        self.target = remote_path.as_ref().to_path_buf();
    }
    /// Set target path after file upload
    pub fn with_remote_path<P: AsRef<Path>>(mut self, remote_path: P) -> Self {
        self.target = remote_path.as_ref().to_path_buf();
        self
    }
    /// Execute this upload task
    pub async fn execute(self) -> QResult<()> {
        let mut scp = self.session.scp_send(&self.target, self.permission, self.content.len() as u64, None)?;
        scp.write(&self.content)?;
        Ok(())
    }
}

impl TryFrom<&Path> for ContentResolver {
    type Error = QError;
    fn try_from(path: &Path) -> QResult<Self> {
        let mut file = File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        Ok(Self { content })
    }
}

impl TryFrom<&PathBuf> for ContentResolver {
    type Error = QError;
    fn try_from(path: &PathBuf) -> QResult<Self> {
        ContentResolver::try_from(path.as_path())
    }
}

impl TryFrom<&[u8]> for ContentResolver {
    type Error = QError;
    fn try_from(data: &[u8]) -> QResult<Self> {
        Ok(Self { content: data.to_vec() })
    }
}

impl TryFrom<&str> for ContentResolver {
    type Error = QError;
    fn try_from(data: &str) -> QResult<Self> {
        Ok(Self { content: data.as_bytes().to_vec() })
    }
}
