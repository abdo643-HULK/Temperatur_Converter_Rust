use std::io::{self, Write};

fn main() {
    println!("Celsius(C)/Fahrenheit(F) Converter!");
    loop{
        let mut unit = String::with_capacity(1);
        print!("Please input the unit to convert to (F or C): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut unit).expect("Please input C or F");
        match unit.trim().to_uppercase().as_str() {
            "C"  => { 
                convert_to_c();
                break;
            },
            "F" => { 
                convert_to_f();
                break;
            },
            _ => continue
        };
    }
}

fn convert_to_c () {
    loop {
        let mut temp = String::new();
        print!("Please input the fahrenheit: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut temp).expect("Please input a number");
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };        
        let f = (temp - 32.0) * 5.0 / 9.0;
        println!("The temperature in Celcius is : {}", f);
        return;
    }
}

fn convert_to_f () {
    loop {
        let mut temp = String::new();
        print!("Please input the celsius: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut temp).expect("Please input a number");
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let c = temp * 9.0/5.0 + 32.0;
        println!("The temperature in Celcius is : {}", c);
        return;
    }
}
