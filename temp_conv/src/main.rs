use std::io;

fn main() {
    println!("Please input your temperature");
    let temp: f64 = get_temp();

    let temp_to_type: String = get_temp_type();
    let answer: f64 = convert_temp(temp_to_type, temp);
    println!("Your input temp is {temp}, & the answer is {answer}");
}

fn get_temp_type() -> String {
    println!("Enter the temperature type to convert to...");

    loop {
        let mut temp_type = String::new();

        match io::stdin().read_line(&mut temp_type) {
            Ok(_) => {},
            Err(err) => {
                println!("Failed to read your temperature type, try again");
                println!("Err {err}");
                continue;
            }
        }

        let parsed_temp_type: String = temp_type.trim().to_lowercase();
        if parsed_temp_type != "c" && parsed_temp_type != "celsius" && parsed_temp_type != "f" && parsed_temp_type != "fahrenheit" {
            println!("Invalid temperature type, try again...");
            continue;
        }

        return parsed_temp_type;
    }
}

fn get_temp() -> f64 {
    'loop_label: loop {
        let mut read_temp: String = String::with_capacity(100000);
        io::stdin().read_line(&mut read_temp).expect("Failed to read temperature");
        
        let _: f64 = match read_temp.trim().parse() {
            Ok(num) => return num,
            Err(err) => {
                println!("Failed to parse your input temperature, please input a valid number");
                println!("Err {err}");
                continue 'loop_label;
            }
        };
    }
}

fn convert_temp(temp_type_to: String, temp: f64) -> f64 {
    if temp_type_to == "c" {
        return ((temp - 32.0) * 5.0) / 9.0;
    } else if temp_type_to == "f" {
        return (temp * 1.8) + 32.0;
    }

    return temp;
}
