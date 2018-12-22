use std::cmp::Ordering;
use std::fmt;
use std::io;

#[derive(Debug, Clone)]
struct Roman(String);

impl Into<u16> for Roman {
    fn into(self) -> u16 {
        const CONVERSION: [(u16, &str); 14] = [
            (50, "L"),
            (40, "XL"),
            (30, "XXX"),
            (20, "XX"),
            (10, "X"),
            (9, "IX"),
            (8, "VIII"),
            (7, "VII"),
            (6, "VI"),
            (5, "V"),
            (4, "IV"),
            (3, "III"),
            (2, "II"),
            (1, "I"),
        ];

        let mut number = 0;
        let mut string_index = 0;
        while string_index < self.0.len() {
            match CONVERSION.iter().find(|(_, sym)| self.0[string_index..].starts_with(*sym)) {
                Some((num, sym)) => {
                    string_index += sym.len();
                    number += num;
                },
                None => {}
            }
        }
        number
    }
}

impl Ord for Roman {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_num: u16 = self.clone().into();
        let other_num: u16 = other.clone().into();
        self_num.cmp(&other_num)
    }
}

impl PartialOrd for Roman {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Roman {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Roman {}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct RoyalName(String, Roman);
impl fmt::Display for RoyalName {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.0, (self.1).0)
    }
}
impl RoyalName {
    fn new(name: &str) -> Self {
        let tokens: Vec<&str> = name.trim().split(' ').collect();
        RoyalName(tokens.first().unwrap().to_string(), Roman(tokens.last().unwrap().to_string()))
    }
}

fn read_line() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).and_then(|_| Ok(input.trim_right().to_string()))
}

fn main() {
    let mut names = Vec::new();
    loop {
        let input = read_line().unwrap();
        if input.len() == 0 {
            break
        }
        names.push(RoyalName::new(&input));
    }
    names.sort();
    names.iter().for_each(|n| println!("{}", n))
}


