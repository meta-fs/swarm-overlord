use super::*;

impl ArtifactAddress for LocalArtifact {
    fn download_link(&self) -> Url {
        todo!()
    }
}

pub struct LocalArtifact {
    /// unix relative path
    relative_path: String,
}
