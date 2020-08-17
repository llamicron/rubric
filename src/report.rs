/// This module is responsible for printing the rubric
/// and submission after grading
use paris::Logger;

use crate::Rubric;


/// Prints a very short report of the rubric and submission, with
/// only necessary information.
///
/// ## Example
/// (Color obviously can't be shown in this example)
/// ```text
/// ℹ Rubric Name
/// ✔ Deadline: 2021-08-13 Fri 00:00:00 -05:00
///
/// ✔ First Criteria	Passed!
/// ✔ Second Criteria	good job!
/// ✔ Third Criteria	yeah!
///
/// ℹ 1 criteria hidden
/// ℹ Grade: 79/80
/// ```
pub fn short(mut rubric: &mut Rubric) {
    let mut log = Logger::new();

    components::rubric_name(&rubric);
    components::deadline(&rubric);

    log.newline(1);
    components::short_criteria(&mut rubric);
    log.newline(1);

    components::hidden(&rubric);
    components::grade(&rubric);
}


pub fn long(mut rubric: &mut Rubric) {
    let mut log = Logger::new();

    components::rubric_name(&rubric);
    log.newline(1);

    components::deadline(&rubric);
    components::allow_late(&rubric);
    components::daily_penalty(&rubric);
    components::final_deadline(&rubric);
    log.newline(1);

    components::long_criteria(&mut rubric);

    components::hidden(&rubric);
    components::grade(&rubric);
    components::current_time();
}


/// All of these functions just print a different piece of the rubric or submission.
/// I want to add color and styles to the output, so it gets a little more complicated
/// than you'd think. This also helps us have different levels of verbosity when printing.
mod components {
    use paris::Logger;
    use chrono::Local;
    use crate::{Rubric, TIMESTAMP_FORMAT};

    pub fn rubric_name(rubric: &Rubric) {
        Logger::new().info(format!("<bold>{}</>", rubric.name));
    }

    pub fn deadline(rubric: &Rubric) {
        let mut log = Logger::new();
        if let Some(deadline) = rubric.deadline {
            if rubric.past_due() {
                log.error(format!("Deadline: <red>{}</>", deadline.format(TIMESTAMP_FORMAT)));
            } else {
                log.success(format!("Deadline: {}", deadline.format(TIMESTAMP_FORMAT)));
            }
        }
    }

    pub fn final_deadline(rubric: &Rubric) {
        let mut log = Logger::new();
        if let Some(deadline) = rubric.final_deadline {
            if rubric.past_due() {
                log.error(format!("Final Deadline: <red>{}</>", deadline.format(TIMESTAMP_FORMAT)));
            } else {
                log.success(format!("Final Deadline: {}", deadline.format(TIMESTAMP_FORMAT)));
            }
        }
    }

    pub fn daily_penalty(rubric: &Rubric) {
        if rubric.daily_penalty > 0 {
            Logger::new().info(format!("Late penalty per day: {}", rubric.daily_penalty));
        }
    }

    pub fn allow_late(rubric: &Rubric) {
        let mut log = Logger::new();
        if rubric.allow_late {
            log.info(format!("Late submission allowed with {} point penalty", rubric.late_penalty));
        } else {
            log.info("Late submission not allowed");
        }
    }

    pub fn short_criteria(rubric: &mut Rubric) {
        for crit in rubric.sorted() {
            crit.print_short();
        }
    }

    pub fn long_criteria(rubric: &mut Rubric) {
        for crit in rubric.sorted() {
            crit.print_long();
            println!();
        }
    }

    pub fn grade(rubric: &Rubric) {
        let mut log = Logger::new();
        if rubric.points() as isize >= rubric.total_points() {
            log.success(format!("<bold>Grade: <green>{}/{}</>", rubric.points(), rubric.total_points()));
        } else {
            log.info(format!("<bold>Grade: {}/{}</>", rubric.points(), rubric.total_points()));
        }
    }

    pub fn hidden(rubric: &Rubric) {
        let mut log = Logger::new();
        let mut hidden = 0;
        // I know about Iterator::fold() but it's more complicated imo
        for crit in rubric.criteria() {
            if crit.hide {
                hidden += 1;
            }
        }

        if hidden > 0 {
            log.info(format!("{} criteria hidden", hidden));
        }
    }

    pub fn current_time() {
        let now = Local::now();
        Logger::new().info(
            format!("Submitted at {}", now.format(TIMESTAMP_FORMAT))
        );
    }
}
