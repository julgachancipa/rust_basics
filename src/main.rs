fn main() {
    let nombre: &str = "Juliana";
    let mut age: u8 = 21;

    age = age + 1;

    println!("Hello my name is {} and I'm {} years old", age, nombre);

    let min_temp: i8 = -5;
    let max_temp: u8 = 10;

    println!("min temp: {}", min_temp);
    println!("max temp: {}", max_temp);
}
