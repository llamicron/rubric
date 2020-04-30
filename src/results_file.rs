use std::path::{PathBuf, Path};
use std::fs::{canonicalize, OpenOptions};
use std::io;

#[derive(Debug)]
pub struct ResultsFile {
    pub path: PathBuf
}

impl ResultsFile {
    /// Creates a new `ResultsFile`, creating the file if necessary.
    ///
    /// A file will be created at the given path. The path provided is anything
    /// that can be converted from to a [`Path`][path], so [`Path`][path],
    /// [`PathBuf`][pathbuf], or `&str` will all work.
    ///
    /// If the file already exists, it will still use that file. The extension will
    /// be set to csv, possibly overwriting a provided extension. This will return
    /// a [`std::io::Error`][err] if the file, for one reason or another,
    /// cannot be created.
    ///
    /// [err]: std::io::Error
    /// [path]: std::path::Path
    /// [pathbuf]: std::path::PathBuf
    ///
    /// ## Example
    /// ```rust
    /// let rf = ResultFile::new("my_results_file.csv").expect("Couldn't create results file");
    /// // write some records
    /// ```
    pub fn new<P: AsRef<Path>>(path: P) -> Result<ResultsFile, io::Error> {
        // Create the file if it doesn't already exist
        let _ = OpenOptions::new().append(true).create(true).open(&path)?;
        // Get the full canonical path to the file path provided
        let mut full_path = canonicalize(path)?;
        // Set the extension to csv if it isn't already
        full_path.set_extension("csv");

        Ok(ResultsFile {
            path: full_path
        })
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{canonicalize, remove_file, create_dir};

    fn test_dir() -> PathBuf {
        let path = canonicalize(".").expect("test_data dir missing. Are you in the right directory?");
        let mut dir = PathBuf::from(path);
        dir.push("test_data/");
        match create_dir(&dir) {
            Ok(_) => return dir,
            Err(_) => return dir
        };

    }

    fn delete<P: AsRef<Path>>(file: P) {
        match remove_file(file) {
            Ok(_) => {},
            Err(_) => {}
        }
    }

    #[test]
    fn test_new_results_file_creates_file() {
        let mut file = test_dir();
        file.push("results_file.csv");

        // From a PathBuf
        let rf = ResultsFile::new(&file).unwrap();
        assert!(rf.path.to_str().unwrap().contains("results_file.csv"));

        delete(file);
    }

    #[test]
    fn test_works_with_abs_or_rel_path() {
        // Relative path
        let rel = PathBuf::from("./test_data/rel.csv");
        assert!(!rel.exists());
        assert!(rel.is_relative());
        let _ = ResultsFile::new(&rel).expect("Couldn't create results file");
        assert!(rel.exists());

        delete(&rel);


        // Absolute path
        let mut abs = PathBuf::from(canonicalize("./test_data/").unwrap());
        abs.push("abs.csv");
        assert!(!abs.exists());
        assert!(!abs.is_relative());
        let _ = ResultsFile::new(&abs).expect("Couldn't create results file");
        assert!(abs.exists());

        delete(&abs);

        let slice = "./test_data/str.csv";
        let _ = ResultsFile::new(&slice);
        let slice_buf = PathBuf::from(&slice);
        assert!(slice_buf.exists());
        delete(slice_buf);
    }

    #[test]
    fn test_overwrite_extension() {
        let path = "./test_data/ext.png";
        let rf = ResultsFile::new(path).expect("Couldn't create results file");
        if let Some(ext) = rf.path.extension() {
            assert_eq!(ext, "csv");
        }
        delete(path);
    }
}
