use std::env;
mod booth;
use booth::Booth;
use booth::complement;

fn main() {
    let mut booth = Booth::new("1101".to_string(), "1011".to_string());
    booth.print();
    booth.add();
    booth.print();
    booth.subtract();
    booth.print();
    booth.shift();
    booth.print();
    booth.shift();
    booth.print();
    booth.shift();
    booth.print();
    booth.shift();
    booth.print();
}

#[test]
fn test_complement() {
    let string = "011010111";
    assert_eq!(complement(string), "100101000");
    
    let string = "0000";
    assert_eq!(complement(string), "1111");

    let string = "1111";
    assert_eq!(complement(string), "0000");
}

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