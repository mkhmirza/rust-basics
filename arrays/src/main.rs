fn main() {
    // arrays can be declared using
    // let array_name: [type; length] = [];
    let x: [i32;5] = [1, 2, 3, 4, 5];
    println!("x: {:?}", x);
    // length of an array 
    println!("Len(x): {}", x.len());

    // arrays can declared 
    let y: [i32; 10] = [1;10];
    println!("y: {:?}", y);
    println!("Len(y): {}", y.len());

    // bytes an array allocates
    println!("Bytes(x): {}", std::mem::size_of_val(&x));
    println!("Bytes(y): {}", std::mem::size_of_val(&y));
}
