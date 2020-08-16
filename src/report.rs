/// This module is responsible for printing the rubric
/// and submission after grading


/// -- Long --
/// name
/// desc
/// deadline
/// allow_late
/// late_penaly
/// daily_penalty
/// final_deadline
///
/// criteria
///
/// hidden
/// grade / total
/// submission time
/// late?

/// -- Short --
/// name
/// deadline (red if late, green otherwise)
/// criteria (green for passed, red otherwise)
/// hidden
/// grade (green if full)

use paris::Logger;

use crate::{Rubric, Submission};


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
pub fn short(mut rubric: &mut Rubric, sub: &Submission) {
    let mut log = Logger::new();

    components::rubric_name(&rubric);
    components::deadline(&rubric);

    log.newline(1);
    components::short_criteria(&mut rubric);
    log.newline(1);

    components::hidden(&rubric);
    components::grade(&rubric, &sub);
}



/// All of these functions just print a different piece of the rubric or submission.
/// I want to add color and styles to the output, so it gets a little more complicated
/// than you'd think. This also helps us have different levels of verbosity when printing.
mod components {
    use paris::Logger;
    use crate::{Rubric, Submission, dropbox::submission::default_timestamp_format};

    pub fn rubric_name(rubric: &Rubric) {
        Logger::new().info(format!("<bold>{}</>", rubric.name));
    }

    pub fn deadline(rubric: &Rubric) {
        let mut log = Logger::new();
        if let Some(deadline) = rubric.deadline {
            if rubric.past_due() {
                log.error(format!("Deadline: <red>{}</>", deadline.format(&default_timestamp_format())));
            } else {
                log.success(format!("Deadline: {}", deadline.format(&default_timestamp_format())));
            }
        }
    }

    pub fn short_criteria(rubric: &mut Rubric) {
        for crit in rubric.sorted() {
            crit.print_short();
        }
    }

    pub fn grade(rubric: &Rubric, sub: &Submission) {
        let mut log = Logger::new();
        if sub.grade >= rubric.total_points() {
            log.success(format!("<bold>Grade: <green>{}/{}</>", sub.grade, rubric.total_points()));
        } else {
            log.info(format!("<bold>Grade: {}/{}</>", sub.grade, rubric.total_points()));
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
}
