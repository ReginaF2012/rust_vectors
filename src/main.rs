// vectors are stored in the heap which means their size does not need to be known at compile time, so they can shrink and grow.
fn main() {
    // create a vector, and tell rust that it will store i32 types
    let v1: Vec<i32> = Vec::new();

    // vec! is a macro for creating a vector, and without declaring the type, Rust will infer that this vector contains i32 types because that is the default integer type
    let v2 = vec![1, 2, 3];
    println!("Hello, world!");

    // modifying a vector
    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let v4 = vec![1, 2, 3, 4, 5];

    // the two ways to get the third element are by using & and [], which gives us a reference, or by using the get method with the index passed as an argument, which gives us an Option<&T>.
    let third: &i32 = &v4[2];
    println!("The third element is {}", third);

    match v4.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v5 = vec![1, 2, 3, 4, 5];

    // The line below would cause the compiler to panic, because there aren't 100 elements in v5. You can still use it if the desired behavior is that your program will crash.

    // let does_not_exist = &v5[100];

    // this will just return None, however that means that the code will have to accomodate for None values
    let does_not_exist = v5.get(100);

    // for loop
    let v6 = vec![100, 32, 57];
    for i in &v6 {
        println!("{}", i);
    }

    // make changes during for loop
    let mut v7 = vec![100, 32, 57];
    for i in &mut v7 {
        // To change the value that the mutable reference refers to, we have to use the dereference operator (*) to get to the value in i before we can use the += operator.
        *i += 50;
    }

    // using an enums to store elements of different types in a vector, because technically each of these will be the SpreadsheetCell type rather than the types stored inside
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // initialize a vector with a set of values. This vector will have 5 0s
    let vec = vec![0; 5];


    // use vector as a stack
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        // Prints 3, 2, 1
        println!("{}", top);
    }

    // initialize a vector with a predetermined capacity. The length will be 0 and if we push more than 10 elements into the vector we will have to reallocate it.
    let mut vec = Vec::<i32>::with_capacity(10);

    // calling .capacity() on a vect will tell you how much capacity has been allotted to it

    // reserves more capacity for a vector, preventing frequent reallocations. After calling this capacity will be greater than or equal to the vector length + additional
    let mut vec2 = vec![1];
    vec2.reserve(10);

    // reserves EXACTLY vector length + additional
    let mut vec3 = vec![1];
    vec3.reserve_exact(10);

    // decreases capacity as much as possible to fit elements currently in the vect
    let mut vec4 = Vec::with_capacity(10);
    vec4.extend([1, 2, 3].iter().cloned());
    assert_eq!(vec4.capacity(), 10);
    vec4.shrink_to_fit();

    // trunctates vect, keeping only elements specified
    let mut vec5 = vec![1, 2, 3, 4, 5];
    vec5.truncate(2);
    assert_eq!(vec5, [1, 2]);
}
