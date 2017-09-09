use std::io;

fn main() {
    println!("°F to °C or °C to °F?");
    println!("1 = °F to °C.");
    println!("2 = °C to °F");
    println!("0 = quit");
    println!("Choose your destiny:");

    let mut correct_input = false;
    let mut input : u8 = 0;

    while !correct_input {
        let mut input_ = String::new();
        io::stdin().read_line(&mut input_).expect("Fatal failure...");
        input = match input_.trim().parse(){
                Ok(num) => {
                    if num < 3{
                        correct_input = true;
                        num
                    }
                    else{
                        println!("Please enter a valid choice!");
                        continue;
                    }
                },
                Err(_) => {
                    println!("Please enter a valid choice!");
                    continue;
                },
        };
    }
    if input == 1 || input == 2{
        correct_input = false;
        let mut t : f64 = 0.0;
        println!("Temperature to convert:");
        while !correct_input {
            let mut input_ = String::new();
            io::stdin().read_line(&mut input_).expect("Fatal failure...");
            t = match input_.trim().parse(){
                    Ok(num) => {
                        correct_input = true;
                        num
                    },
                    Err(_) => {
                        println!("Please enter a number!");
                        continue;
                    },
            };
        }
        if input == 1{
            println!("{} °F = {} °C", t, convert(input, t));
        }
        else{
            println!("{} °C = {} °F", t, convert(input, t));
        }
        convert(input, t);
    }
    println!("Goodbye for now!");
}

fn convert(mode : u8, t : f64) -> f64{
    if mode == 1{
        (t - 32.0) * 5.0 / 9.0
    }
    else{
        t * 9.0 / 5.0 + 32.0
    }
}
