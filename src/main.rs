use std::collections::HashMap;
use std::io;

fn main(){
    println!("Please enter the name of the employee");
    let mut employee_name=String::new();
    io::stdin().read_line(&mut employee_name).expect("Failed to read line");
    let mut employee=employee_name.trim();
    println!("Please enter the name of the department of the employee");
    let mut department_name=String::new();
    io::stdin().read_line(&mut department_name).expect("Failed to read line");
    let mut department: &str=department_name.trim();
    let mut add_employee_details=HashMap::new();
    add_employee_details.insert(&mut employee, &mut department);
    println!("{:?}",add_employee_details);
}