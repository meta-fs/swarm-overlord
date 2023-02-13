use pubgrub::{
    range::Range,
    solver::{resolve, OfflineDependencyProvider},
    version::SemanticVersion,
};

use crate::ArtifactAddress;

pub struct Dependency {}

impl Dependency {}

pub struct SwarmArtifact {
    pub name: String,
    pub version: SemanticVersion,
    pub content: Box<dyn ArtifactAddress>,
    pub hash: String,
    pub size: u64,
}

pub struct FileNode {
    path: String,
}

#[test]
pub fn a() {
    let mut dependency_provider = OfflineDependencyProvider::<&str, SemanticVersion>::new();
    dependency_provider.add_dependencies(
        "root",
        SemanticVersion::new(1, 0, 0),
        vec![("menu", Range::any()), ("icons", Range::any())],
    );
    dependency_provider.add_dependencies("menu", SemanticVersion::new(1, 0, 0), vec![("dropdown", Range::any())]);
    dependency_provider.add_dependencies("dropdown", SemanticVersion::new(1, 0, 0), vec![("icons", Range::any())]);
    dependency_provider.add_dependencies("icons", SemanticVersion::new(1, 0, 0), vec![]);

    // Run the algorithm.
    let solution = resolve(&dependency_provider, "root", SemanticVersion::new(1, 0, 0)).unwrap();
    println!("Solution: {:?}", solution);
}
