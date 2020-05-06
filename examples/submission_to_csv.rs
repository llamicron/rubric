extern crate lab_grader;

use lab_grader::*;


fn main() {
    let mut sub = Submission::new("luke", 1234);
    sub.use_data(data! {
        "akey1" => "value1",
        "ckey3" => "value3",
        "bkey2" => "value2"
    });

    print!("{}", sub.header());
    println!("{}", sub.as_csv());
}
