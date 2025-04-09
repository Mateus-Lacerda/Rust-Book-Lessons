use std::io::stdin;

fn convert(temp: i8) -> f32
{
    ((temp as f32) - 32.0) * (5.0/9.0)
}

pub fn main()
{
    let mut temp = String::new();
    println!("Give me a temperature in Fahrenheit and see what happens...");
    let _size = stdin().read_line(&mut temp).unwrap();
    // shadow opt 
    let temp: i8 = temp.trim().parse().expect("Please give me numbers!");
    let new_temp = convert(temp);
    println!("Your new temperature: {new_temp}");
}
