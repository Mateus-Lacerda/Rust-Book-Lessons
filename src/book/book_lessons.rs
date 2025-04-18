#![allow(deprecated, unused_variables)]
// This will be awesome

// Variables are immutable by default
fn immutable_variables() {
    let x = 5;
    println!("The value of x is: {x}");
    //x = 6; // does not compile!!!!
    let mut y = 6;
    println!("The value of y is: {y}");
    y = 2;
    println!("The value of y is: {y}");
}

// Constants are always immutable
fn constants() {
    // const x = 4; // Does not compile!!!! Needs typing
    // const x: i32 = 10; // WARNING: Constants must be named with upper snake case
    const X: i32 = 42; // This is perfect
    println!("{X}");
    // X = 1; this also does not compile
}

// Shadowing is very cool indeed
fn make_shadows(){
    let x = 5;

    // creates new variable, overriding the one previously defined
    // Doesn't need to me mut!
    let x = x + 1;

    {
        // This only modifies the value of x inside these curly brackets
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // Still 6
    println!("The value of x is: {x}");
    
    // You may also change the variable's type with shadowing
    let spaces = "   ";
    println!("{spaces}");

    let spaces = spaces.len();
    println!("{spaces}")
}

// DATA TYPES OH YEAH

// what is u32??
#[deprecated(since="0.0.71", note="I now know wth is u32")]
fn wth_is_u32() {
    // The compiler can't infer the types when we parse from another type
    // So u32 seems to do the job
    // ... but why not i32? // Found out, check line 71 (18 j in vim normal mode)
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}")
}

// Scalar types
// integers, floating-point numbers, Booleans, and characters
// Represent a single value

// INTEGERS
// Length	Signed	Unsigned
// 8-bit	i8	    u8
// 16-bit	i16 	u16
// 32-bit	i32 	u32
// 64-bit	i64 	u64
// 128-bit	i128	u128
// arch	    isize	usize
//
// The signed ones reserve a bit to indicate posivitity ;)
// arch depends on the system's architecture (x32, x64) // known at running time, noice

// Literals synthax
fn int_literals_synthax() {
    let decimal: u16 = 1_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // u8 only
    
    // lets have some fun
    println!("{}", (decimal + hex));
    println!("{}", (octal + binary));
    println!("{}", byte);
}
// INTEGER OVERFLOW
// If you make stuff with ints that makes their value bigger
// than reserved mem syze the program panics!
// Overflow means that the value will do a full loop
// and go back to zero if it exceeds by 1 the max val
//let x: u8 = 255;
//let x: u8 = x + 1; // please no, crabs panic!

// Float types
fn float_types () {
    let x = 2.0; // f64

    println!("{x}");

    let y: f32 = 3.0; // f32
    println!("{y}");
}

// Finally some math 
//
// (The prints here were beatifully written with a vim macro)
fn make_the_calcs() {
    // addition
    let sum = 5 + 10;
    println!("{sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{difference}");

    // multiplication
    let product = 4 * 30;
    println!("{product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("{quotient}");

    // truncated
    let truncated = -5 / 3; // Results in -1
    println!("{truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");
}

// The boolean
// The one you already know
fn let_the_bools_cook() {
    let t = true;
    assert!(t);

    let f: bool = false;
    assert!(!f);
}

// The char
//
fn charring() {
    let c = 'z';
    println!("{c}");
    let z: char = 'ℤ';
    println!("{z}");
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");
}


// Compound types
// They group multiple values into one type 

// The tuple
// General way of grouping different values
// Allow different types
// Are a default in Rust, unlike C
fn the_tup() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Deconstructing the tuple into many variables
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    // Access a value directly
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!("{five_hundred}");

    let six_p_four = x.1;
    println!("{six_p_four}");

    let one = x.2;
    println!("{one}");


    // The unit
    // Functions without return, return this
    // Like this one
    let the_unit = ();
}

// The Array
// Allocated in the stack
// What does that even mean???
// More useful when the number of elements is not suposed to change
fn the_array() {
    let a = [1, 2, 3, 4, 5];
    // for loop just like Python
    for number in a {
        println!("{number}")
    }

    // Typing the array: [<type>;len]
    let b: [i8; 4] = [-1, 0, 3, 42];
    for number in b {
        println!("{number}")
    }

    // Specify the value that you want repeated
    // arr = [<element to repeat>;<lenght>]
    // 4 42's in sequence
    let fourty_twos = [42;4];
    for fourty_two in fourty_twos {
        println!("{fourty_two}")
    }
    // accessing an element
    let fst_42 = fourty_twos[0];
    let lst_42 = fourty_twos[3]; // Cant use -1

    // If you try to acces an invalid element 
    // Doesn't compile
    // If index is not known at compile time (user input)
    // Panic !!!
    //
    // Other compiled languages let you access the suposed data
    // Which can result in memory unsafety

}

// Functions
// Main is the entrypoint, must normally be present
// Naming is in snake_case
// No need for return statement, just place the variable lmao
fn get_me_the_value_of(x: i8) -> i8 {x}

//Statements are instructions that perform some action and do not return a value.
//Expressions evaluate to a resultant value. Let’s look at some examples.
fn statements_and_instructions() { // The function's definition is a statement
    let y = 6; // Statement
    println!("{y}");
    // Statements don't return values, hence:
    // let x = (let y = 6); // Does not compile
    // Python is laughing right now
    let x = {
        let k = 10;
        k + 1 // Semicolon is not allowed here, since it is interpreted as a statement
    };
    println!("The value of x is {x}");
}

// Comments (This is a comment)

// Control flow
// Loops and ifs
fn out_of_control_flow() {
    let x = "dumb";
    // If statements
    if x == "smart" {
        println!("Smart")
    } else if x == "normal" {
        println!("Normal")
    } else if x == "dumb" {
        println!("Dumb, as expected")
    } else {
        println!("What")
    }
    // Using if is let statement
    // Very Pythonic indeed
    let dumb = true;
    let smart = if dumb { "dumb" } else { "smart" };
    // Values in ifs in lets must have same type
    // This does NOT compile:
    //    let condition = true;
    //
    //    let number = if condition { 5 } else { "six" };
    //
    //    println!("The value of number is: {number}");

    // Loops
    let mut counter = 0;
    // Basically a nice old while true
    loop {
        if counter == 5 {break};
        println!("again");
        counter += 1;
    }
    // Loops may calculate stuff, and therefore return stuff:
    let mut number = 0;
    let result = loop {
        number += 1;

        if number == 10 {
            break number * 2;
        }
    };
    println!("{number}"); // Must be 20
}

// WARNING: Very cool stuff ahead
// Labelling loops
fn loop_labeling() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {break};
            if count == 2 {break 'counting_up};
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}")
}

// While 
fn the_while_loop() {
    let mut number = 3;
    
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LISTOFF");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    } // This, apparently, is very dangerous

    for element in a {
        println!("{element}") // Much simple imo
    }

    // Range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

pub fn run_book_lessons(){
    println!("\nRunning book lessons");
    immutable_variables();
    constants();
    make_shadows();
    wth_is_u32();
    int_literals_synthax();
    float_types();
    make_the_calcs();
    let_the_bools_cook();
    charring();
    the_tup();
    the_array();
    let _x = get_me_the_value_of(10);
    statements_and_instructions();
    out_of_control_flow();
    loop_labeling();
    the_while_loop();
}

