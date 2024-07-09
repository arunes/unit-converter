pub mod temperature;

enum ConversionUnit {
    Unknown,
    Temperature,
}

pub struct Conversion {
    value: f32,
    from: String,
    to: String,
    unit: ConversionUnit,
}

impl Conversion {
    pub fn new(value: f32, from: &String, to: &String) -> Conversion {
        let from = from.to_string().to_uppercase();
        let to = to.to_string().to_uppercase();

        let temp_units: [&str; 2] = ["C", "F"];
        let unit = match (&from, &to) {
            (f, t) if temp_units.contains(&f.as_str()) && temp_units.contains(&t.as_str()) => {
                ConversionUnit::Temperature
            }
            _ => ConversionUnit::Unknown,
        };

        Conversion {
            value,
            from,
            to,
            unit,
        }
    }

    pub fn convert(&self) {
        match self.unit {
            ConversionUnit::Temperature => {
                let result = temperature::convert(self.value, &self.from, &self.to);
                println!("{result}{}", self.to);
            }
            ConversionUnit::Unknown => {
                println!("Cannot convert, unkown unit.");
            }
        }
    }
}
