use std::io;

fn main() {
    println!("Test fahrenheit_to_celsius -> {}", fahrenheit_to_celsius(22.0));
    println!("Test celsius_to_fahrenheit -> {}", celsius_to_fahrenheit(6.0));

    println!("Please, select the input type is celsius or fahrenheit (c/f)!");

    let mut temp_type   =   String::new();
    let mut temprature  =   String::new();

    io::stdin()
        .read_line(&mut temp_type)
        .expect("Fail to read type!");

    println!("Please give the temprature!");

    io::stdin()
        .read_line(&mut temprature)
        .expect("Failt to read temprature!");

    let temprature: f32 = temprature.trim().parse().expect("Something went wrong!");
    let _cel = 'c';
    let _fahr = 'f';

    match temp_type {
        _cel => println!("Celsius to Fahrenheit -> {}", celsius_to_fahrenheit(temprature)),
        _fahr => println!("Celsius to Fahrenheit -> {}", fahrenheit_to_celsius(temprature))
    };
    
}

fn fahrenheit_to_celsius(far: f32) -> f32 {
    let tmp = {far - 32 as f32};
    tmp as f32  / 1.8000
}

fn celsius_to_fahrenheit(cel: f32) -> f32 {
    let tmp = {cel as f32 * 1.8000};
    tmp + 32 as f32
}
