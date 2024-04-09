use std::fmt::Error;
use std::fs::File;
use std::io::{self, Write};
use std::io::BufRead;
mod booth;
use booth::Booth;

use crate::booth::decimal_to_binary_string;

fn main() {
    // let mut booth = Booth::new("11001".to_string(), "10111".to_string());
    // booth.extended_solve();
    // println!("");
    // let mut booth = Booth::new("11111001".to_string(), "11100111".to_string());
    // booth.solve();
    // let numbers = read_binary_from_file("input.txt");
    // match numbers {
    //     Ok(numbers) => {
    //         for pair in numbers {
    //             // println!("{} {}", pair.0, pair.1);
    //             let mut booth = Booth::new(pair.0.to_string(), pair.1.to_string());
    //             println!("NORMAL");
    //             booth.solve();
    //             let mut booth = Booth::new(pair.0.to_string(), pair.1.to_string());
    //             println!("EXTENDED");
    //             booth.extended_solve();
    //             println!("");

    //         }
    //     },
    //     Err(_) => todo!(),
    // } 

    // export_normal_data();
    export_extended_data();
}

fn read_binary_from_file(filename: &str) -> io::Result<Vec<(String, String)>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut binary_hex_pairs = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.trim().split(' ');

        if let Some(binary1) = parts.next() {
            if let Some(binary2) = parts.next() {
                binary_hex_pairs.push((binary1.to_string(), binary2.to_string()));
            }
        }
    }

    Ok(binary_hex_pairs)
}

fn export_normal_data() -> io::Result<()> {
    let mut file = File::create("normal_output.txt")?;
    let numbers = read_binary_from_file("input.txt");
    match numbers {
        Ok(numbers) => {
            Ok(for pair in numbers {
                let mut booth = Booth::new(pair.0.to_string(), pair.1.to_string());
                booth.solve();
                let _ = file.write(format!("{} {} {} {}\n", pair.0.len(), booth.iterations, booth.additions, booth.subtractions).as_bytes());

            })
        },
        Err(_) => todo!(),
    } 
}

fn export_extended_data() -> io::Result<()> {
    let mut file = File::create("extended_output.txt")?;
    let numbers = read_binary_from_file("input.txt");
    match numbers {
        Ok(numbers) => {
            Ok(for pair in numbers {
                let mut booth = Booth::new(pair.0.to_string(), pair.1.to_string());
                booth.extended_solve();
                let _ = file.write(format!("{} {} {} {}\n", pair.0.len(), booth.iterations, booth.additions, booth.subtractions).as_bytes());

            })
        },
        Err(_) => todo!(),
    } 
}
// #[test]
// fn test_complement() {
//     let string = "011010111";
//     assert_eq!(complement(string), "100101000");
    
//     let string = "0000";
//     assert_eq!(complement(string), "1111");

//     let string = "1111";
//     assert_eq!(complement(string), "0000");
// }

#[test]
fn test_add() {
    let mut booth = Booth::new("1010".to_string(), "1010".to_string());
    booth.add();
    assert_eq!(booth.q, "1010");

    booth.add();
    assert_eq!(booth.q, "0100");

    booth.add();
    assert_eq!(booth.q, "1110");
}

#[test]
fn test_subtract() {
    let mut booth = Booth::new("1010".to_string(), "1010".to_string());
    booth.subtract();
    assert_eq!(booth.q, "0110");

    booth.subtract();
    assert_eq!(booth.q, "1100");

    booth.subtract();
    assert_eq!(booth.q, "0010");
}

#[test]
fn test_solve() {
    let mut booth = Booth::new("1010".to_string(), "1010".to_string());
    booth.solve();
    assert_eq!(booth.q, "0010");
}

#[test]
fn test_extended_solve () {
    let mut booth = Booth::new("1010".to_string(), "1010".to_string());
    booth.extended_solve();
    assert_eq!(booth.q, "0010");
}

#[test]
fn test_length_4() {
    let base: i64 = 2;
    let length: u32 = 4;
    let range = base.pow(length);
    let half_range = range / 2;
    for i in -half_range..range-half_range {
        for j in -half_range..range-half_range {
            println!("i: {}", i);
            println!("j: {}", j);
            println!("i: {}", decimal_to_binary_string(i, length as usize));
            println!("j: {}", decimal_to_binary_string(j, length as usize));
            let mut booth = Booth::new(decimal_to_binary_string(i, length as usize), decimal_to_binary_string(j, length as usize));
            booth.solve();
            assert_eq!(booth.get_answer().abs(), (i * j).abs());
        }
    }
}

#[test]
fn test_length_8 () {
    let base: i64 = 2;
    let length: u32 = 8;
    let range = base.pow(length);
    let half_range = range / 2;
    for i in -half_range..range-half_range {
        for j in -half_range..range-half_range {
            println!("i: {}", i);
            println!("j: {}", j);
            println!("i: {}", decimal_to_binary_string(i, length as usize));
            println!("j: {}", decimal_to_binary_string(j, length as usize));
            let mut booth = Booth::new(decimal_to_binary_string(i, length as usize), decimal_to_binary_string(j, length as usize));
            booth.solve();
            assert_eq!(booth.get_answer().abs(), (i * j).abs());
        }
    }
}


// fn main() {
//     // Get command-line arguments
//     let args: Vec<String> = env::args().collect();

//     // Check if there are exactly two arguments (excluding the program name)
//     if args.len() != 3 {
//         println!("Usage: {} <binary_number1> <binary_number2>", args[0]);
//         return;
//     }

//     // Parse binary numbers from command-line arguments
//     let x = args[1].to_string();
//     let binary1 = match u32::from_str_radix(&args[1], 2) {
//         Ok(num) => num,
//         Err(_) => {
//             println!("Error: Invalid binary number '{}'", args[1]);
//             return;
//         }
//     };

//     let y = args[2].to_string();
//     let binary2 = match u32::from_str_radix(&args[2], 2) {
//         Ok(num) => num,
//         Err(_) => {
//             println!("Error: Invalid binary number '{}'", args[2]);
//             return;
//         }
//     };

//     // Print the binary numbers
//     println!("Binary number 1: {} {}", binary1, x);
//     println!("Binary number 2: {} {}", binary2, y);
// }