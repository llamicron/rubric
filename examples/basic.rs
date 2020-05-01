#[macro_use]
extern crate lab_grader;

fn main() {
    let input = prompt!("enter something: ", String);
    println!("{}", input);
}
