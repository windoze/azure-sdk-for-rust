use crate::config_parser::Tag;
use camino::Utf8Path;
use std::{
    fs::File,
    io::{prelude::*, LineWriter},
};

pub type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(crate::io::Error),
}
impl<T: Into<crate::io::Error>> From<T> for Error {
    fn from(error: T) -> Self {
        Self::Io(error.into())
    }
}

pub fn create(crate_name: &str, tags: &[&Tag], default_tag: &Tag, path: &Utf8Path) -> Result<()> {
    let file = File::create(path)?;
    let mut file = LineWriter::new(file);
    let default_feature = default_tag.rust_feature_name();

    // https://docs.rs/about/metadata
    // let docs_rs_features = docs_rs_features(tags, &default_feature);
    // let docs_rs_features: Vec<_> = docs_rs_features.iter().map(|s| format!("\"{}\"", s)).collect();
    // let docs_rs_features = docs_rs_features.join(", ");

    file.write_all(
        format!(
            r#"# generated by AutoRust
[package]
name = "{}"
version = "0.3.0"
edition = "2021"
license = "MIT"
description = "generated REST API bindings"
repository = "https://github.com/azure/azure-sdk-for-rust"
homepage = "https://github.com/azure/azure-sdk-for-rust"
documentation = "https://docs.rs/{}"

[dependencies]
azure_core = {{ path = "../../../sdk/core", version = "0.2", default-features = false }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
bytes = "1.0"
http = "0.2"
url = "2.2"
futures = "0.3"

[dev-dependencies]
azure_identity = {{ path = "../../../sdk/identity" }}
tokio = {{ version = "1.0", features = ["macros"] }}
env_logger = "0.9"
chrono = "0.4"

[package.metadata.docs.rs]
all-features = true

[features]
default = ["{}", "enable_reqwest"]
enable_reqwest = ["azure_core/enable_reqwest"]
enable_reqwest_rustls = ["azure_core/enable_reqwest_rustls"]
no-default-tag = []
"#,
            crate_name, crate_name, default_feature
        )
        .as_bytes(),
    )?;

    for tag in tags {
        file.write_all(format!("\"{}\" = []\n", tag.rust_feature_name()).as_bytes())?;
    }
    Ok(())
}

pub fn get_default_tag<'a>(tags: &[&'a Tag], default_tag: Option<&str>) -> &'a Tag {
    if let Some(default_tag) = default_tag {
        if let Some(tag) = tags.iter().find(|tag| tag.name() == default_tag) {
            return tag;
        }
    }
    let tag = tags.iter().find(|tag| !tag.name().contains("preview"));
    match tag {
        Some(tag) => tag,
        None => tags[0],
    }
}

// const MAX_DOCS_RS_FEATURES: usize = 4;
// const NO_DEFAULT_TAG: &str = "no-default-tag";

// /// Get a list of features to document at docs.rs in addition the default
// fn docs_rs_features(tags: &[&Tag], default_feature: &str) -> Vec<String> {
//     let mut features: Vec<_> = tags
//         .iter()
//         .filter_map(|tag| {
//             let feature = tag.rust_feature_name();
//             if feature == default_feature {
//                 None
//             } else {
//                 Some(feature)
//             }
//         })
//         .collect();
//     features.truncate(MAX_DOCS_RS_FEATURES);
//     features.insert(0, NO_DEFAULT_TAG.to_owned());
//     features
// }
