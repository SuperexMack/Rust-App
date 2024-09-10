use std::io;


fn main(){
    add_student();
}

fn add_student(){


    let mut students : Vec<(String,i32)> = Vec::new();

   loop {

    println!("Enter the details of the students || press exit to just leave the application");

    let mut  name_of_student : String = String::new();
    let mut roll_no : String = String::new();
    println!("Enter the name of the student");
    // now we are going to input the name of the student
    io::stdin().read_line(&mut name_of_student).expect("Enter a valid name || it is not a string");
    let name_of_student:String = name_of_student.trim().to_string();

    if name_of_student == "exit"{
        break;
    }

    println!("Enter the roll no. of the student");
    io::stdin().read_line(&mut roll_no).expect("number entered by you is invalid");
    let roll_no:i32 = roll_no.trim().parse().expect("Roll number is not valid");

    // println!("The name of the student is : {name_of_student}");
    // println!("Roll number of the student is : {roll_no}");

    // now we are going to make a tuple here so that we can add roll number and names of the students

    students.push((name_of_student,roll_no));

   }

   println!("\nList of student is");

   for(name,rollno) in students {
    println!("Name - {name} and roll no. {rollno}");
   }

    
   
}