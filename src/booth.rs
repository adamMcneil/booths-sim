pub(crate) struct Booth {
    pub q: String, // represents the extra register to complete the multiplication
    pub e: char,   // extra bit
    pub a: String, // a and b are the two numbers that are being multiplied
    pub b: String,
    pub length: usize,
}

impl Booth {
    pub fn new(a: String, b: String) -> Booth {
        let length = a.len();
        Booth {
            a: a,
            b: b,
            q: std::iter::repeat('0').take(length).collect(),
            e: '0',
            length: length,
        }
    }

    pub fn print(&self) {
        println!("q: {}, a: {}, e: {}, b: {}", self.q, self.a, self.e, self.b);
    }

    pub fn solve(&mut self) {
        let n = self.length; 

        for _ in 0..n {
            let bit = self.a.chars().last();

            match bit {
                Some('0') => {
                    if self.e == '1' {
                        self.add();
                    }
                },
                Some('1') => {
                    if self.e == '0' {
                        self.subtract();
                    }
                },
                _ => {},
            }
            self.shift();
            self.print();
        }
    }

    pub fn add(&mut self) {
        self.q = add_binary_strings(&self.q, &self.b, self.length)
    }

    pub fn subtract(&mut self) {
        self.q = subtract_binary_strings(&self.q, &self.b, self.length)
    }

    pub fn shift(&mut self) {
        self.e = self.a.pop().unwrap();
        self.a = format!("{}{}", self.q.pop().unwrap(), self.a);
        self.q = format!("{}{}", self.q.chars().next().unwrap(), self.q);
    }
}

fn binary_string_to_decimal(binary: &str) -> i64 {
    i64::from_str_radix(binary, 2).expect("Invalid binary string")
}

fn decimal_to_binary_string(decimal: i64) -> String {
    format!("{:b}", decimal)
}

fn add_binary_strings(a: &str, b: &str, length: usize) -> String {
    let sum = binary_string_to_decimal(a) + binary_string_to_decimal(b);
    let mut sum_string = decimal_to_binary_string(sum);
    if sum_string.len() > length {
        sum_string = sum_string.chars().skip(1).collect();
    }
    sum_string
}

fn subtract_binary_strings(a: &str, b: &str, length: usize) -> String {
    let sum = binary_string_to_decimal(a) - binary_string_to_decimal(b);
    let mut sum_string = decimal_to_binary_string(sum);
    if sum_string.len() > length {
        sum_string = sum_string[sum_string.len() - length..].to_string();
    }
    sum_string
}

pub(crate) fn complement(a: &str) -> String {
    let complemented: String = a
    .chars()
    .map(|c| {
        if c == '0' {
            '1'
        } else if c == '1' {
            '0'
        } else {
            panic!("Invalid input! Input must be a binary string.")
        }
    })
    .collect();

    complemented
}
