extern crate lab_grader;

use std::collections::HashMap;
use lab_grader::criterion::Criterion;

fn main() {
    let mut crits: Vec<Criterion> = vec![
        // These are all in-line. See `examples/abstracted.rs` for
        // a better way to organize these.
        Criterion::new(
            "My first criterion",
            10,
            ("file is there", "file is missing"),
            Box::new(|_: &HashMap<String, String>| {
                // do some super smart calculations here
                true
            })
        ),
        Criterion::new(
            "My second criterion",
            5,
            ("web server responded", "web server didn't respond"),
            Box::new(|_: &HashMap<String, String>| {
                // do some super smart calculations here
                true
            })
        ),
        Criterion::new(
            "My first criterion",
            15,
            ("ran the proper command", "didn't run the proper command"),
            Box::new(|_: &HashMap<String, String>| {
                // do some super smart calculations here
                false
            })
        ),
        Criterion::new(
            "My first criterion",
            25,
            ("pass", "fail"),
            Box::new(|_: &HashMap<String, String>| {
                // do some super smart calculations here
                true
            })
        ),
    ];

    for crit in &mut crits {
        crit.test();
        println!("{}", crit);
    }
}
