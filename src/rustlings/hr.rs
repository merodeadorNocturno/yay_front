/*
* Complete the 'gradingStudents' function below.
*
* The function is expected to return an INTEGER_ARRAY.
* The function accepts INTEGER_ARRAY grades as parameter.
1. Student 1 received a 73, and the next multiple of 5 from 73 is 75. Since 75 - 73 < 3, the student's grade is rounded to 75.
2. Student 2 received a 67, and the next multiple of 5 from 67 is 70. Since 70 - 67 = 3. the grade will not be modified and the
student's final grade is 67.
3. Student 3 received a 38, and the next multiple of 5 from 38 is 40. Since 40 - 38 < 3, the student's grade will be rounded to 40.
4. Student 4 received a grade below 33, so the grade will not be modified and the student's final grade is 33.
*/

fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut my_vec: Vec<i32> = Vec::new();
    let no_rounding_grade = 37;
    let threshold = 3;
    let multiple_factor = 5;

    for grade in grades.iter().skip(0) {
        let modulo = grade % multiple_factor;
        let next_multiple_of_five = grade + multiple_factor - modulo;
        if (next_multiple_of_five - grade < threshold) & (grade > &no_rounding_grade) {
            my_vec.push(next_multiple_of_five);
        }

        if (next_multiple_of_five - grade >= threshold) & (grade > &no_rounding_grade) {
            my_vec.push(grade.to_owned());
        }

        if grade <= &no_rounding_grade {
            my_vec.push(grade.to_owned());
        }
    }
    my_vec
}
