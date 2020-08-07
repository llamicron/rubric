# Overview
This framework provides tools for instructors to write student-run grading applications, called "graders". Graders are written by instructors and distributed to students. When run by the students, the grader will assess their work and send a report to the dropbox, which is maintained by the instructor.

There are 3 significant parts to a grading application
1. A **rubric** - a list of specific criteria that each student must fulfil
1. **Submissions** - An evaluation of a specific student's work and progress through a rubric
1. A **dropbox** - a collection box for submissions

## Rubrics
Rubrics are exactly what you would expect; a list of criteria that describe an assignment or test. Each criteria has a point value, and the students grade is determined by the criteria they fulfil.

## Submissions
A submission is a bundle of data that tracks how well the student performed, according to the rubric. Rubrics and submissions are useless without each other. Submissions are meant to be *submitted* (who knew?) to the instructor. 

## The Dropbox
The dropbox is simply a web server maintained by the instructor that recieves submissions and records them. The submissions are recorded as CSV, which is supported by Microsoft Excel. Sorting and manipulation can be performed in Excel after all the submissions are in.
