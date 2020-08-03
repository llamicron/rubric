// std uses
use std::fmt;
use std::process::Command;
use std::str::FromStr;

// external uses
use regex::Regex;

pub enum Program {
    Git,
    Docker,
    Python,
    Ruby,
    DockerCompose,
    // AzureCLI,
}

impl Program {
    /// Returns the version number of the program,
    /// or None if it isn't installed.
    pub fn version(self) -> Option<Version> {
        Version::of(self)
    }
}

/// Represents a programs version.
///
/// You probably don't want to build this directly, see the
/// [`Program`](crate::helpers::system::Program) enum.
#[derive(Debug, PartialEq)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    /// Returns the installed version of the program provided.
    /// ```rust
    /// use rubric::system::{Version, Program};
    ///
    /// // Tests that git is installed
    /// assert!(Version::of(Program::Git).is_some());
    /// ```
    pub fn of(program: Program) -> Option<Self> {
        // If it returned a version
        if let Some(s) = Self::get_string(program) {
            if let Ok(v) = s.parse::<Self>() {
                return Some(v);
            }
        }

        None
    }

    /// Makes a custom version number. Mostly use to compare to another
    ///
    /// ```rust
    /// # use rubric::system::Version;
    /// #
    /// let v = Version::custom(4, 5, 6);
    /// assert_eq!(v.major(), 4);
    /// assert_eq!(v.minor(), 5);
    /// assert_eq!(v.patch(), 6);
    /// ```
    pub fn custom(major: u32, minor: u32, patch: u32) -> Self {
        Version {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version (first number)
    pub fn major(&self) -> u32 {
        self.major
    }

    /// Returns the minor version (second number)
    pub fn minor(&self) -> u32 {
        self.minor
    }

    /// Returns the patch version (third number)
    pub fn patch(&self) -> u32 {
        self.patch
    }

    /// Returns the string version of a program
    ///
    /// This is private, don't call this.
    fn get_string(program: Program) -> Option<String> {
        use Program::*;

        // Get command and regex pattern based on program
        let (cmd, pattern) = match program {
            Git => ("git --version", r"(\d+\.\d+\.\d+)"),
            Docker => ("docker -v", r"(\d+\.\d+\.\d+)"),
            DockerCompose => ("docker-compose -v", r"(\d+\.\d+\.\d+)"),
            Python => ("python --version", r"(\d+\.\d+\.\d+)"),
            Ruby => ("ruby -v", r"(\d+\.\d+\.\d+)"),
        };

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", cmd]).output()
        } else {
            Command::new("sh").arg("-c").arg(cmd).output()
        };

        if let Ok(resp) = output {
            let re: Regex = pattern.parse().unwrap();

            let text = match String::from_utf8(resp.stdout) {
                Ok(t) => t,
                Err(_) => return None,
            };

            if let Some(cap) = re.captures(&text) {
                if let Some(version) = cap.get(1) {
                    return Some(version.as_str().to_owned());
                }
            }
        }

        None
    }
}

impl FromStr for Version {
    type Err = std::num::ParseIntError;

    /// This splits by ".". Any values not found are set to 0.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(".").collect();

        let major_str = parts.get(0).unwrap_or(&"0");
        let minor_str = parts.get(1).unwrap_or(&"0");
        let patch_str = parts.get(2).unwrap_or(&"0");

        let major = major_str.parse::<u32>()?;
        let minor = minor_str.parse::<u32>()?;
        let patch = patch_str.parse::<u32>()?;

        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let string = "4.5.63";
        let v: Version = string.parse().unwrap();
        assert_eq!(v.major(), 4);
        assert_eq!(v.minor(), 5);
        assert_eq!(v.patch(), 63);
    }

    #[test]
    fn test_partial_version_failure() {
        let string = "3.7";
        let v: Version = string.parse().unwrap();
        assert_eq!(v.major(), 3);
        assert_eq!(v.minor(), 7);
        assert_eq!(v.patch(), 0);

        let string2 = "3";
        let v2 = string2.parse::<Version>();
        assert!(v2.is_ok());
    }

    #[test]
    fn test_version_of() {
        let v = Version::of(Program::Git);
        assert!(v.is_some());
    }

    #[test]
    fn test_version_display() {
        let v = Version::custom(1, 2, 3);

        assert_eq!("1.2.3", &format!("{}", v));
    }

    #[test]
    fn test_program_version_from_enum() {
        assert!(Program::Git.version().is_some());
    }
}
