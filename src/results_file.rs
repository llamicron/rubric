// std uses
use std::path::{PathBuf, Path};
use std::fs::{File, canonicalize, OpenOptions, metadata};
use std::io::{self, Write};


/// Trait to convert a struct to csv (comma separated values).
///
/// You should not append a newline for any of these functions.
///
/// ## Example Implementation
/// ```rust
/// use rubric::results_file::AsCsv;
///
/// // A dummy struct so we can impl AsCsv
/// pub struct Point {
///     x: i32,
///     y: i32
/// }
///
/// impl AsCsv for Point {
///     fn as_csv(&self) -> String {
///         format!("{},{}", self.x, self.y)
///     }
///
///     fn filename(&self) -> String {
///         String::from("points.csv")
///     }
///
///     fn header(&self) -> String {
///         String::from("x,y")
///     }
/// }
///
/// let p = Point { x: 4, y: 8 };
/// assert_eq!(p.header(), "x,y");
/// assert_eq!(p.filename(), "points.csv");
/// assert_eq!(p.as_csv(), "4,8");
/// ```
pub trait AsCsv {
    /// The item in CSV format. This should *not* append a newline.
    fn as_csv(&self) -> String;
    /// The filename where this type should be saved.
    /// Usually this should just be `<item>.csv`
    fn filename(&self) -> String;
    /// The header for the csv file. Should match the fields
    /// in `as_csv()`
    fn header(&self) -> String;
}

/// A CSV results file containing the results of the grading process.
#[derive(Debug)]
pub struct ResultsFile {
    pub path: PathBuf,
    handle: File
}

impl ResultsFile {
    /// Creates a new `ResultsFile`, creating the file if necessary.
    ///
    /// **Note**: You probably shouldn't use this. Instead, try `ResultsFile::for_item` below.
    ///
    /// A file will be created at the given path, and write the given header.
    /// The path provided is anything that can be converted from to a
    /// [`Path`][path], so [`Path`][path], [`PathBuf`][pathbuf], or `&str` will all work.
    ///
    /// If the file already exists, it will still use that file. This will return
    /// a [`std::io::Error`][err] if the file, for one reason or another,
    /// cannot be created.
    ///
    /// [err]: std::io::Error
    /// [path]: std::path::Path
    /// [pathbuf]: std::path::PathBuf
    ///
    /// ## Example
    /// ```rust
    /// use rubric::results_file::ResultsFile;
    ///
    /// let rf = ResultsFile::new("my_results_file.csv", "").expect("Couldn't create results file");
    /// # use std::fs::remove_file;
    /// # remove_file("my_results_file.csv").unwrap();
    /// ```
    pub fn new<P: AsRef<Path>, S: AsRef<str>>(path: P, header: S) -> Result<ResultsFile, io::Error> {
        // Create the file if it doesn't already exist
        let handle = OpenOptions::new().append(true).create(true).open(&path)?;
        // Get the full canonical path to the file path provided
        let full_path = canonicalize(path)?;

        let mut rf = ResultsFile {
            path: full_path,
            handle
        };
        if rf.length() == 0 {
            if let Err(e) = rf.append(&header.as_ref()) {
                return Err(io::Error::from(e));
            }
        }
        Ok(rf)
    }

    pub fn for_item<I: AsCsv>(item: &I) -> Result<ResultsFile, io::Error> {
        ResultsFile::new(item.filename(), item.header())
    }

    /// Returns the length of the results file in bytes.
    ///
    /// This will panic if the file doesn't exist or if this process
    /// does not have permission to access it. The file is created by this
    /// process when making a new `ResultsFile`, so as long as you don't change
    /// the file permissions or delete the file while your program is running,
    /// you'll be fine.
    ///
    /// ## Example
    /// ```rust
    /// # use rubric::results_file::ResultsFile;
    /// let rf = ResultsFile::new("file.csv", "123").unwrap();
    ///
    /// assert_eq!(rf.length(), 4);
    /// # use std::fs::remove_file;
    /// # remove_file("file.csv").unwrap();
    /// ```
    pub fn length(&self) -> u64 {
        let m = metadata(&self.path).expect("File does not exist or this process does not have permission to access it");
        m.len()
    }

    /// Appends the given `&str` to the file, with a trailing newline.
    ///
    /// Returns an `io::Result` containing the size written.
    /// `ResultsFile` must be mutable.
    ///
    /// ## Example
    /// ```rust
    /// # use rubric::results_file::ResultsFile;
    /// let mut rf = ResultsFile::new("append.csv", "").unwrap();
    ///
    /// assert_eq!(rf.length(), 1);
    /// if let Err(e) = rf.append("here's some content") {
    ///     // Something went wrong, deal with it
    /// }
    /// assert!(rf.length() > 0);
    /// # use std::fs::remove_file;
    /// # remove_file("append.csv").unwrap();
    /// ```
    pub fn append(&mut self, record: &str) -> io::Result<usize> {
        let to_write = format!("{}\n", record);
        self.handle.write(to_write.as_bytes())
    }

    /// Writes an item to the csv file in csv format. This item must implement
    /// the [AsCsv][ascsv] trait.
    ///
    /// This method *does* append a newline after the record is written. Again,
    /// the results file will need to be mutable.
    ///
    /// [ascsv]: crate::results_file::AsCsv
    /// ## Example
    /// ```rust
    /// # use rubric::results_file::{ResultsFile, AsCsv};
    /// # struct Point { x: i32, y: i32 };
    /// # impl AsCsv for Point {
    /// #     fn as_csv(&self) -> String { format!("{},{}", self.x, self.y) }
    /// #     fn filename(&self) -> String { String::from("points.csv") }
    /// #     fn header(&self) -> String { String::from("x,y") }
    /// # }
    /// // A custom struct that implements AsCsv
    /// let point = Point { x: 6, y: 19 };
    ///
    /// let mut rf = ResultsFile::for_item(&point).unwrap();
    /// assert_eq!(rf.length(), 4);
    /// if let Err(e) = rf.write_csv(&point) {
    ///     // Something went wrong, deal with it
    /// }
    /// assert!(rf.length() > 4);
    /// # use std::fs::remove_file;
    /// # remove_file(point.filename()).unwrap()
    /// ```
    pub fn write_csv<R: AsCsv>(&mut self, record: &R) -> io::Result<usize> {
        self.append(&format!("{}", record.as_csv()))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{canonicalize, remove_file, create_dir};

    pub struct Point {
        x: i32,
        y: i32
    }

    impl AsCsv for Point {
        fn as_csv(&self) -> String {
            format!("{},{}", self.x, self.y)
        }

        fn filename(&self) -> String {
            String::from("points.csv")
        }

        fn header(&self) -> String {
            String::from("x,y")
        }
    }

    fn header() -> String {
        String::from("x,y")
    }

    fn test_dir() -> PathBuf {
        let path = canonicalize(".").expect("test_data dir missing. Are you in the right directory?");
        let mut dir = PathBuf::from(path);
        dir.push("test_data");
        create_dir(&dir).ok();
        return dir;
    }

    fn delete<P: AsRef<Path>>(file: P) {
        remove_file(file).ok();
    }

    #[test]
    fn test_new_results_file_creates_file() {
        let mut file = test_dir();
        file.push("results_file.csv");

        // From a PathBuf
        let rf = ResultsFile::new(&file, header()).unwrap();
        assert!(rf.path.to_str().unwrap().contains("results_file.csv"));

        delete(file);
    }

    #[test]
    fn test_works_with_abs_or_rel_path() {
        // Relative path
        let rel = PathBuf::from("./test_data/rel.csv");
        assert!(!rel.exists());
        assert!(rel.is_relative());
        let _ = ResultsFile::new(&rel, header()).expect("Couldn't create results file");
        assert!(rel.exists());

        delete(&rel);


        // Absolute path
        let mut abs = PathBuf::from(canonicalize("./test_data/").unwrap());
        abs.push("abs.csv");
        assert!(!abs.exists());
        assert!(!abs.is_relative());
        let _ = ResultsFile::new(&abs, header()).expect("Couldn't create results file");
        assert!(abs.exists());

        delete(&abs);

        let slice = "./test_data/str.csv";
        let _ = ResultsFile::new(&slice, header());
        let slice_buf = PathBuf::from(&slice);
        assert!(slice_buf.exists());
        delete(slice_buf);
    }

    #[test]
    fn test_get_length() {
        let mut file = test_dir();
        file.push("length.csv");
        let rf = ResultsFile::new(&file, header()).unwrap();
        assert_eq!(rf.length(), 4);
        delete(&file);
    }

    #[test]
    fn test_append() {
        let content = "here's some content to write";
        let mut file = test_dir();
        file.push("append.csv");
        let mut rf = ResultsFile::new(&file, header()).unwrap();
        assert_eq!(rf.length(), 4);
        rf.append(&content).expect("Couldn't write to results file");
        rf.append(&content).expect("Couldn't write to results file");
        rf.append(&content).expect("Couldn't write to results file");
        assert!(rf.length() > 3);

        delete(&file);
    }

    #[test]
    fn test_write_csv() {
        let mut file = test_dir();
        file.push("write_csv.csv");

        let point = Point { x: 5, y: 7 };

        let mut rf = ResultsFile::new(&file, header()).unwrap();
        assert_eq!(rf.length(), 4);

        let result = rf.write_csv(&point);

        assert!(result.is_ok());
        assert!(rf.length() > 3);

        delete(&file);
    }

    #[test]
    fn test_results_file_for_csv_item() {
        let point = Point { x: 32, y: 37 };
        let rf = ResultsFile::for_item(&point).expect("Couldn't make file");
        assert!(format!("{}", rf.path.display()).contains(&point.filename()));
        delete(point.filename());
    }
}
