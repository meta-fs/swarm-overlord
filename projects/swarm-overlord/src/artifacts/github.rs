use super::*;

pub struct GithubArtifact {}

impl ArtifactAddress for GithubArtifact {
    fn download_link(&self) -> Url {
        todo!()
    }
}
