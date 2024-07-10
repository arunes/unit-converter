use converters::Conversion;
use regex::Regex;
use std::env;

pub mod converters;

fn main() {
    if let Some(parsed) = parse_args() {
        parsed.convert();
    } else {
        println!("Enter arguments as `cv 20C to F`. This will convert 20 celcius to Fahrenheit.");
    }
}

fn parse_args() -> Option<Conversion> {
    let args: Vec<String> = env::args().collect();
    let combined = args[1..].join(" ");

    let regex = Regex::new(r"(?<value>((-|\+)?\d*\.?\d*))(?<from>\w+) to (?<to>\w+)");
    if let Ok(re) = regex {
        let captures = re.captures(&combined);
        if let Some(c) = captures {
            return Some(Conversion::new(
                c["value"].parse::<f32>().unwrap(),
                &c["from"].to_string(),
                &c["to"].to_string(),
            ));
        }
    }

    None
}
