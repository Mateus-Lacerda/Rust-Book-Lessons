#![allow(deprecated)]
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
}

