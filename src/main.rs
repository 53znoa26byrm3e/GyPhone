// std lib
use std::io;
use std::fs::File;
use std::io::Write;

fn main() {

    let mut prefix: String = String::new();
    println!("What is the prefix of the country : ");
    io::stdin().read_line(&mut prefix).unwrap().to_string();

    let mut number: String = String::new();
    println!("How many numbers after the prefix : ");
    io::stdin().read_line(&mut number).expect("Error while reading number");
    let number = match number.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Enter a number");
            return;
        }
    };

    let mut filename: String = String::new();
    println!("Output file : ");
    io::stdin().read_line(&mut filename).unwrap();
    let mut file = File::create(filename.trim()).unwrap();

    let mut max_number = String::new();
    for _ in 0..number {
        max_number.push('9');
    }
    let max_number: u32 = match max_number.trim().parse() {
        Ok(max_number) => max_number,
        Err(_) => {
            println!("Error while converting");
            return;
        }
    };

    let numb = number;

    for i in 10..=max_number {
        let format = format!("{:0width$}", i, width = numb);
        let result = format!("{}{}", prefix.trim(), format);
        file.write_all(result.as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    }
}