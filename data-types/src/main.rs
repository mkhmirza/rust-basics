fn main() {
    // integer type
    let x = 20;
    // string type
    let name = "John Doe";
    // float/double type
    let  pi = 3.1452;
    // character/char type
    let ch = 'a';
    // values of let variable_name = default value cannot be changed
    // uncommenting the line below will throw an error
    //ch = 'b';
    // for changing the value of a varable see mutable

    
    println!("Integer: {}", x);
    println!("String: {}", name);
    println!("Float: {}", pi);
    println!("Single Character: {}", ch);

    // type can defined at the declaration
    // let variable_name: type = value;
    let integer: i32 = 400;
    let bools: bool = true;

    println!("Integer: {}", integer);
    println!("Bools: {}", bools);

    // varables can also be declared first
    // rust uses snake_case as defualt naming convention
    let first_name;
    let last_name;
    // initializing the declared variable
    first_name = "John";
    last_name = "Doe";
    println!("First Name: {}", first_name);
    println!("Last Name: {}", last_name);

    // a mutable variable can change its value after its initialization
    // let mut varible_name = value
    let mut mutable = 23;

    println!("Original Value: {}", mutable);
    mutable = 45;
    println!("Value Change: {}", mutable);
   
    // you cannot change the type of the mutable variable
    // mutable = true;

    // overshowding is allowed
    let mutable = true;
    println!("New Type: {}", mutable);

    // can also have a predefined type
    let mut can_drive: bool;
    can_drive = true;
    println!("Can Drive: {}", can_drive);
    can_drive = false;
    println!("Can Drive - Changed: {}", can_drive);

}
