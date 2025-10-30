
enum Grade {
    A,
    B,
    C,
    D,
    F,
}

struct Student {
    name: String,
    grade: Grade,
    marks: u32,
}

fn show_result(student: &Student) {
    println!("Name: {}", student.name);
    println!("Marks: {}", student.marks);

    match student.grade {
        Grade::A => println!("Grade: A - Excellent! "),
        Grade::B => println!("Grade: B - Good job! "),
        Grade::C => println!("Grade: C - Fair performance."),
        Grade::D => println!("Grade: D - Needs improvement."),
        Grade::F => println!("Grade: F - Failed "),
    }
}

fn main() {

    let student1 = Student {
        name: String::from("charan"),
        grade: Grade::A,
        marks: 92,
    };

    let student2 = Student {
        name: String::from("sai"),
        grade: Grade::C,
        marks: 68,
    };

    let student3 = Student {
        name: String::from("ram"),
        grade: Grade::f,
        marks: 32,
    };

    show_result(&student1);
    println!("----------");
    show_result(&student2);
    println!("----------");
    show_result(&student3);
}
