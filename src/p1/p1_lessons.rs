fn lesson1() {
    let x: i32;
    let y: i32;
    y = 5;
    x = y;
    assert_eq!(x, 5);
    println!("Lesson 1: Sucess!");
}

fn lesson2() {
    // mut says that the value may be changed
    let mut x: i32 = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Lesson 2: Success!");
}

fn lesson3() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
    println!("Lesson 3: Success!");
}

fn lesson4() {
    fn define_x() {
        let x: &str = "hello";
        println!("{}, world", x);
    }
    define_x();
    println!("Lesson 4: Success!");
}

fn lesson5() {
    let x: i32 = 4;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 4);

    let x: i32 = 42;
    println!("{}", x);
    println!("Lesson 5: Success!");
}

fn lesson6() {
    let mut _x: i32 = 1;
    _x = 7;
    _x += 3;

    let _y = 4;

    let _y = "I can also be bound to text!";

    println!("Lesson 6: Success!");
}

fn lesson7() {
    fn sol1() {
        let _x = 1;
        println!("Solution 1: Success!");
    }
    fn sol2() {
        let x = 2;
        println!("{}", x);
        println!("Solution 2: Success!");
    }
    sol1();
    sol2();
    println!("Lesson 7: Success!");
}

fn lesson8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Lesson 8: Success!");
}

fn lesson9() {
    let (x, y);

    (x,..) = (3,4);
    [..,y] = [1, 2];
    assert_eq!([x,y], [3,2]);
    println!("Lesson 9: Success!");
}

pub fn run_p1() {
    lesson1();
    lesson2();
    lesson3();
    lesson4();
    lesson5();
    lesson6();
    lesson7();
    lesson8();
    lesson9();
    println!("You now now a little more of Rust's types and assignments!");
}
