//declaring a struct
struct Employee {
    ceo:String,
    company:String,
    age:u32
}

fn main() {
    //instantiating a function
    let emp1 = Employee {
        company:String::from("Google Inc."),
        ceo:String::from("Satya Nadella"),
        age:56
    };
    let emp2 = Employee {
        company:String::from("Google Inc."),
        ceo:String::from("Sundai Pichai"),
        age:51
    };
    //Pass emp1 and emp2 to uninitialized function display() as object parameters
    display(emp1);
    display(emp2);
}
//fetch values of specific structure fields using
//operator and print it to the console
fn display(emp:Employee) {
    println!("Name is :{} company is {} age is {}",emp.ceo,emp.company,emp.age);
}
