#[derive(Debug)]
struct School {
    student: String,
    roll_no: u32,
}

impl School {
    fn default() -> School {
        School {
            student: String::from("Amal"),
            roll_no: 1,
        }
    }
}

fn main() {
    println!("Static Method");
    let mut student1 = School::default();
    println!("student1: {:?}", student1);
    student1.student = String::from("Akash");
    student1.roll_no = 2;
    println!("student1: {:?}", student1);
}
