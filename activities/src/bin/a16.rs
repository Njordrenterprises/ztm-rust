// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>


// FIRST TRY
// * Use a struct containing the student's name and locker assignment
struct LockerAssignment {
    name: Option<String>,
    number: Option<i32>
}

fn main() {
    let student_locker = LockerAssignment {
        name: Some("Jane".to_owned()),
        number: Some(42),
    };

    match student_locker.name { 
        Some(locker) => println!("Student Name: {:?}", locker),
        None => println!("No data")
    }

    match student_locker.number {
        Some(locker) => println!("Locker Number: {:?}", locker),
        None => println!("They have not selected a locker."), 
    } //dont need commas or colons after matches it seems
}
//close but not as logical as it could be


//more logical so can hold the student data better and shorter code 
struct Student {
    name: String,
    number: Option<i32>,
}

fn main() {
    let john = Student {
        name: "John".to_owned(),
        number: Some(3)
    };

    println!("student: {:?}", john.name);
    match john.number {
        Some(num) => println!("Locker number: {:?}, num"),
        None => println!("no locker assigned"),
    }
}
