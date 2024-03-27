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
        println!("a: {}", binary_string_to_decimal_twos_complement(&self.a));
        println!("b: {}", binary_string_to_decimal_twos_complement(&self.b));

        println!("{} {} {} {}", self.q, self.a, self.e, self.b);
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
            // self.print();
        }
        println!("answer: {}", binary_string_to_decimal_twos_complement(&(self.q.to_string() + &self.a)));
    }

    pub fn extended_solve(&mut self) {
        let n = self.length; 
        println!("a: {}", binary_string_to_decimal_twos_complement(&self.a));
        println!("b: {}", binary_string_to_decimal_twos_complement(&self.b));

        println!("{} {} {} {}", self.q, self.a, self.e, self.b);
        for _ in 0..n/2 {
            let bits = &self.a[self.length - 2..];
            println!("{}", bits); 
            match bits {
                "00" => {
                    if self.e == '1' {
                        self.add();
                    }
                },
                "01" => {
                    if self.e == '0' {
                        self.add();
                    } else {
                        self.add();
                        self.add();
                    }
                },
                "10" => {
                    if self.e == '0' {
                        self.subtract();
                        self.subtract();
                    } else {
                        self.subtract();
                    }
                },
                "11" => {
                    if self.e == '0' {
                        self.subtract();
                    }
                },
                _ => {},
            }
            self.shift();
            self.shift();
        }
        println!("answer: {}", binary_string_to_decimal_twos_complement(&(self.q.to_string() + &self.a)));
    }

    pub fn add(&mut self) {
        self.q = add_binary_strings(&self.q, &self.b, self.length);
        println!("{} {} {} {} ADD", self.q, self.a, self.e, self.b);
    }

    pub fn subtract(&mut self) {
        self.q = subtract_binary_strings(&self.q, &self.b, self.length);
        println!("{} {} {} {} SUB", self.q, self.a, self.e, self.b);
    }

    pub fn shift(&mut self) {
        self.e = self.a.pop().unwrap();
        self.a = format!("{}{}", self.q.pop().unwrap(), self.a);
        self.q = format!("{}{}", self.q.chars().next().unwrap(), self.q);
        println!("{} {} {} {} SHIFT", self.q, self.a, self.e, self.b);
    }

    pub fn get_answer(self) -> i64 {
        binary_string_to_decimal_twos_complement(&(self.q.to_string() + &self.a))
    }
}

fn binary_string_to_decimal(binary: &str) -> i64 {
    i64::from_str_radix(binary, 2).expect("Invalid binary string")
}

pub(crate) fn binary_string_to_decimal_twos_complement(binary: &str) -> i64 {
    let is_negative = binary.chars().next().unwrap() == '1';

    if !is_negative {
        return i64::from_str_radix(binary, 2).unwrap();
    }

    let inverted_binary: String = binary
        .chars()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect();


    -((i64::from_str_radix(&inverted_binary, 2).unwrap()) + 1)
}

pub(crate) fn decimal_to_binary_string(decimal: i64, length: usize) -> String {
    let mut string = format!("{:b}", decimal);
    if string.len() > length {
        string = string[string.len() - length..].to_string();
    }
    while string.len() < length {
        string = format!("0{}", string);
    }
    string
}

fn add_binary_strings(a: &str, b: &str, length: usize) -> String {
    let sum = binary_string_to_decimal_twos_complement(a) + binary_string_to_decimal_twos_complement(b);
    let mut sum_string = decimal_to_binary_string(sum, length);
    if sum_string.len() > length {
        sum_string = sum_string[sum_string.len() - length..].to_string();
    }
    while sum_string.len() < length {
        sum_string = format!("0{}", sum_string);
    }
    sum_string
}

fn subtract_binary_strings(a: &str, b: &str, length: usize) -> String {
    let sum = binary_string_to_decimal_twos_complement(a) - binary_string_to_decimal_twos_complement(b);
    let mut sum_string = decimal_to_binary_string(sum, length);
    if sum_string.len() > length {
        sum_string = sum_string[sum_string.len() - length..].to_string();
    }
    while sum_string.len() < length {
        sum_string = format!("0{}", sum_string);
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
