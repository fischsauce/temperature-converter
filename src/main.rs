use std::io;
// use std::cmp::Ordering;


fn main() {
    
    
    loop {

        // const celcius: u8 = 1;
        // const farenheit: u8 = 2;

        // Display instructions:
        println!("Welcome, please select your INPUT unit:\n");
        println!("1. for Celcius \n2. for Farenheit \n3. to quit");

        // Declare unit type variable and wait for input:
        let mut unit = String::new();
        io::stdin().read_line(&mut unit).expect("Failed to read line");
        // Ensure unit selection is a number between 1..3 (ie 1 or 2):
        let unit: u8 = match unit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue
            },
            
        };

        if unit == 3 {
            break
        }
        
        // Get the input temperature:
        loop {
            println!("Please enter the temperature to convert:");
            
            // Declare temp variable and wait for input:
            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("Failed to read line");
            // Ensure temperature input is a number between 1..100:
            let temp: f64 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    continue
                },
            };

            // Do the conversion:

            // Celcius to Farenheit:
            if unit == 1 {
                let output:f64 = (temp * 1.8) + 32.0;
                // Print the output:
                println!(
                    "\n-------------------\n{:.2}℃  = {:.2}℉\n-------------------\n", temp, output);
                break

            // Farenheit to Celcius:
            } else if unit == 2 {
                let output:f64 = (temp - 32.0) / 1.8;
                // Print the output:
                println!(
                    "\n-------------------\n{:.2}℉  = {:.2}℃\n-------------------\n", temp, output);
                break
            };
        };
    }
}
