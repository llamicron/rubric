//! Criteria built from `yaml`
use std::str;
use rust_embed::RustEmbed;

#[derive(Debug, RustEmbed)]
#[folder = "criteria/"]
struct YamlFile;

impl YamlFile {
    /// Loads a `yaml` file
    ///
    /// "Batch" files are yaml files containing criteria and some metadata. They are
    /// read from the filesystem when compiling for debug, and embedded in the exeutable when
    /// compiling for release.
    pub fn load(batch_name: &str) -> Option<String> {
        let file = YamlFile::get(batch_name)?;
        if let Ok(content) = str::from_utf8(file.as_ref()) {
            return Some(content.to_string());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_yaml_file() {
        let opt = YamlFile::load("test_batch.yml");
        assert!(opt.is_some());
        assert!(opt.unwrap().contains("Hello"));
    }
}
