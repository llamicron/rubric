//! A set of data to verify a Submissions validity

// std uses
use std::env;

// external uses
use serde::{Serialize, Deserialize};

// internal uses
use crate::dropbox::AsCsv;


#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Fingerprint {
    /// Any random string
    pub secret: String,
    pub platform: String
}

impl Fingerprint {

    /// Creates a new fingerprint from a secret string
    pub fn from_secret(secret: &str) -> Self {
        Fingerprint {
            secret: String::from(secret),
            platform: String::from(env::consts::OS)
        }
    }
}

impl AsCsv for Fingerprint {
    fn as_csv(&self) -> String {
        format!("{},{}", self.secret, self.platform)
    }

    fn filename(&self) -> String {
        format!("fingerprint.csv")
    }

    fn header(&self) -> String {
        format!("secret,platform")
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_fingerprint() {
        let fp = Fingerprint::from_secret("my secret key");
        assert_eq!(fp.secret, "my secret key");
        assert!(fp.platform.len() > 0);
    }

    #[test]
    fn test_as_csv() {
        let fp = Fingerprint::from_secret("my_secret");
        assert!(fp.header().contains(","));
        assert!(fp.header().len() > 0);
        assert!(fp.as_csv().contains("my_secret"));
        if cfg!(target_os = "windows") {
            assert!(fp.as_csv().contains("windows"));
        }
    }
}
