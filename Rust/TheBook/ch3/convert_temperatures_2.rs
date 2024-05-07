// Convert temperatures between Fahrenheit and Celsius.

use std::io;

fn convert_temperature_to_fahrenheit(temperature: f64) -> f64 {
    let temperature_in_fahrenheit: f64 = (temperature * 1.8) + 32.0;
    temperature_in_fahrenheit
}

fn convert_temperature_to_celsius(temperature: f64) -> f64 {
    let temperature_in_celsius: f64 = (temperature - 32.0) * 0.5556;
    temperature_in_celsius
}

fn main() {
    loop{
        let mut unit = String::new();
        println!("Enter temperature unit symbol (c or f)\nOr enter q to quit:");
        io::stdin().read_line(&mut unit).unwrap();
        let unit = unit.trim();
        if unit == "q"{
            println!("bye!");
            break
        }else if unit != "c" && unit != "f"{
            println!("Not valid unit symbol. Please, enter c for celsius or f for fahrenheit\nTo quit enter q");
            continue
        }
        
        println!("Enter a temperature");
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("can't read a line");
            let temperature: f64 = match temperature.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

            if unit == "f"{
                let temp_f = convert_temperature_to_celsius(temperature);
            println!("temperature: {temp_f} C");
            }else if unit == "c"{
                let temp_c = convert_temperature_to_fahrenheit(temperature);
            println!("temperature: {temp_c} F");
        }
    }
}
