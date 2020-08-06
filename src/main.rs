use rubric::{Rubric, yaml};

fn main() {
    let yaml = yaml!("../test_data/test_rubric.yml").unwrap();
    let rubric = Rubric::from_yaml(&yaml).unwrap();

    println!("{:#?}", rubric.deadline);
}
