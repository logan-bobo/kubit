#![deny(rustdoc::broken_intra_doc_links, rustdoc::bare_urls, rust_2018_idioms)]

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Kube Error: {0}")]
    KubeError(#[from] kube::Error),

    #[error("OCI error: {0}")]
    OCIParseError(#[from] oci_distribution::ParseError),

    #[error("OCI error: {0}")]
    OCIError(#[from] oci_distribution::errors::OciDistributionError),

    #[error("Unsupported manifest type: Index")]
    UnsupportedManifestIndex,

    #[error("Error decoding package config JSON: {0}")]
    DecodePackageConfig(serde_json::Error),

    #[error("Error decoding kubecfg pack metadata JSON: {0}")]
    DecodeKubecfgPackageMetadata(serde_json::Error),

    #[error("Cannot fetch docker credentials: {0}")]
    CredentialRetrievalError(#[from] docker_credential::CredentialRetrievalError),

    #[error("Error rendering spec back as JSON: {0}")]
    RenderOverlay(serde_json::Error),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    TempfilePersistError(#[from] tempfile::PersistError),

    #[error("Namespace is required")]
    NamespaceRequired,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Expose all controller components used by main.
pub mod controller;

/// Resource type definitions.
pub mod resources;

pub mod apply;
pub mod render;
