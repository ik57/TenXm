//Convert temperatures between Fahrenheit and Celsius.

fn main() {
    let temp_in_celsius: f64 = 33.5;
    let temp_in_fahrenheit: f64 = 86.1;
    let c_to_f: f64 = convert_temperature_to_fahrenheit(temp_in_celsius);
    let f_to_c: f64 = convert_temperature_to_celsius(temp_in_fahrenheit);
    println!("{} celsius is {} fahrenheit", temp_in_celsius, c_to_f);
    println!("{} fahrenheit is {} celsius", temp_in_fahrenheit, f_to_c);
}

fn convert_temperature_to_fahrenheit(temp: f64) -> f64 {
    let temp_in_fahrenheit: f64 = (temp * 1.8) + 32.0;
    temp_in_fahrenheit
}

fn convert_temperature_to_celsius(temp: f64) -> f64 {
    let temp_in_celsius: f64 = (temp - 32 as f64) * 0.5556;
    temp_in_celsius
}
