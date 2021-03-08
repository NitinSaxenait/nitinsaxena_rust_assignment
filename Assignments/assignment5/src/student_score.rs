// structure containing details of students
//
// #Variants
pub struct StudentInformation {
    pub name: String,
    pub roll_no: u32,
    pub(crate) score_of_each_subject: Score,
    pub department: String,
    pub school: String,
}
// structure containing score of different subjects in i32 format
//
// #Variants
pub struct Score {
    pub(crate) hindi: i32,
    pub(crate) english: i32,
    pub(crate) maths: i32,
    pub(crate) science: i32,
}
// function taking students data and structures
//
// #Arguements
//
// returning string
impl StudentInformation {
    pub fn new(score_of_each_subject: &Score) -> StudentInformation {
        StudentInformation {
            name: String::from("Nitin"),
            roll_no: 25,
            department: String::from("Information Technology"),
            school: String::from("Saint School"),
            score_of_each_subject: Score {
                hindi: score_of_each_subject.hindi,
                english: score_of_each_subject.english,
                maths: score_of_each_subject.maths,
                science: score_of_each_subject.science,
            },
        }
    }
    // function making condition of passing marks
    //
    // adding marks as needed marks to make student pass
    //
    // passing marks is 35 for each subject
    pub fn student_passed(&mut self) {
        if self.score_of_each_subject.hindi < 35 {
            let needed_marks = 35 - self.score_of_each_subject.hindi;
            self.score_of_each_subject.hindi += needed_marks;
        }
        if self.score_of_each_subject.english < 35 {
            let needed_marks = 35 - self.score_of_each_subject.english;
            self.score_of_each_subject.english += needed_marks;
        }
        if self.score_of_each_subject.maths < 35 {
            let needed_marks = 35 - self.score_of_each_subject.maths;
            self.score_of_each_subject.maths += needed_marks;
        }
        if self.score_of_each_subject.science < 35 {
            let needed_marks = 35 - self.score_of_each_subject.science;
            self.score_of_each_subject.science += needed_marks;
        }
    }
    // function to get average of marks of student
    //
    // subjects: maths, english, hindi, science
    pub fn get_average(&self) -> f32 {
        let average_of_marks: f32 = ((self.score_of_each_subject.science
            + self.score_of_each_subject.maths
            + self.score_of_each_subject.english
            + self.score_of_each_subject.hindi)
            / 4) as f32;
        average_of_marks
    }
    // function used to compare student marks
    //
    // #Arugments
    //
    // self and second student taken as parameter
    pub fn compare_student(&self, second_student: &Score) {
        println!(
            "For HindiDifference {}",
            (self.score_of_each_subject.hindi - second_student.hindi).abs()
        );
        println!(
            "For english_difference {}",
            (self.score_of_each_subject.english - second_student.english).abs()
        );
        println!(
            "For maths_difference {}",
            (self.score_of_each_subject.maths - second_student.maths).abs()
        );
        println!(
            "For science_difference {}",
            (self.score_of_each_subject.science - second_student.science).abs()
        );
    }
}
