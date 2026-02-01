pub fn run (){
  //BASIC OF STRUCT  Field-by-field initialization
    struct Person {
        name : String,
        id :i32,
        age:u32,
        salary:f32
 }

  {
    let person1 =Person{
      name: String::from ("Raj"),
    age: 20,
    id:888,
    salary:90000.0
    };
    println!("Name:{}",person1.name);
    println!("Age: {}",person1.age);
    println!("Id: {}",person1.id);
    println!("Salary: {}",person1.salary);


  }
 

}