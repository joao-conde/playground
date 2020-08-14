use std::io;

//just a test, could be improved by using enums
fn main() {
    let scale = loop {
        let mut input = String::new();
        println!("Convert temperature from (F or C)?");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read temperature scale used!");
        let input = input.trim().to_uppercase();
        match input.as_str() {
            "F" => break input,
            "C" => break input,
            _ => println!("Pick either (F)ahrenheit or (C)elsius!"),
        };
    };

    let value = loop {
        let mut input = String::new();
        println!("Temperature value?");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read temperature!");
        match input.trim().parse::<i32>() {
            Ok(val) => break val,
            _ => println!("Not a number!"),
        };
    };

    let conversion = match scale.as_str() {
        "F" => (value - 32) as f32 / 1.8,
        "C" => (value as f32) * 1.8 + 32.0,
        _ => -1.0,
    };

    match scale.as_str() {
        "F" => println!("F={}, C={}", value, conversion),
        "C" => println!("C={}, F={}", value, conversion),
        _ => println!("Failed conversion!"),
    };
}
