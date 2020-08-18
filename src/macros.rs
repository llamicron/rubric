//! Various macros


/// This reads data from a file using
/// the include_bytes! macro. When compiling for debug, this will read
/// from the filesystem. When compiling for release, this will embed the data
/// in the executable. Graders built using this crate need to have the data embedded
/// in the executable to make it easier to distribute and to keep the data private.
///
/// Returns `Result<&str, Utf8Error>`.
#[macro_export]
macro_rules! yaml {
    ( $file:expr ) => {
        ::std::str::from_utf8(include_bytes!($file))
    };
}



/// A macro to easily create a [`TestData`](crate::submission::TestData)
/// struct, which is really just an alias to `HashMap<String, String>`.
///
/// ## Example
/// ```rust
/// # extern crate rubric;
/// use rubric::{TestData, data};
///
/// // The long way
/// let mut map = TestData::new();
/// map.insert(String::from("key"), String::from("value"));
///
/// // the macro way
/// let data = data! { "key" => "value" };
/// assert_eq!(map, data);
/// ```
#[macro_export]
macro_rules! data (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert(String::from($key), String::from($value));
            )+
            m
        }
     };
);



/// Attaches tests to criteria in a rubric.
///
/// This will accept a rubric and one or more function names. It will
/// then attempt to find a criterion for each function passed in. The criteria
/// should have a `func` field that matches the name of the function. It will panic
/// if it doesn't find a matching criteria.
///
/// When you create a rubric from `yaml`, the criteria inside
/// don't have tests attached to them. You can call
/// [`Rubric.attach()`](crate::rubric::Rubric::attach) to achieve the
/// same thing, but this is faster and easier.
///
/// ## Example
/// ```no_compile
/// // A test meant to be attached to a criteria
/// fn some_test(_: &TestData) -> bool {
///     true
/// }
///
/// fn main() {
///     let mut rubric = Rubric::from_yaml(/* some yaml data */);
///     // Assuming there is a criterion with:
///     //     func: some_test
///     attach!(rubric, some_test);
///
///     // or be explicit
///     // This is the same thing
///     attach! {
///         rubric,
///         "non_matching_func_key" => some_test
///     }
/// }
/// ```
#[macro_export]
macro_rules! attach {
    // Short way
    ($rubric:ident, $($func:path),*) => {
        $(
            let chunks: Vec<&str> = std::stringify!($func).split("::").collect();
            let func_name = chunks.into_iter().next_back().unwrap();
            if let Some(c) = $rubric.get(func_name) {
                c.attach(Box::new($func));
            } else {
                panic!("Criteria with func `{}` not found. `func` field and function name must match exactly", func_name);
            }
        )+
    };
    // Long way
    ( $rubric:ident, $($func_name:literal => $func:path),* ) => {
        $(
            if let Some(c) = $rubric.get($func_name) {
                c.attach(Box::new($func));
            } else {
                panic!("Criterion with func {} not found, can't attach function", $func_name);
            }
        )+
    };
}
