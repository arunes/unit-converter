pub mod length;
pub mod temperature;

enum ConversionUnit {
    Unknown,
    Temperature,
    Length,
}

pub struct Conversion {
    value: f32,
    from: String,
    to: String,
    unit: ConversionUnit,
}

impl Conversion {
    pub fn new(value: f32, from: &String, to: &String) -> Conversion {
        let from = from.to_string().to_lowercase();
        let to = to.to_string().to_lowercase();

        let temp_units = temperature::get_units();
        let length_units = length::get_units();

        let unit = match (&from, &to) {
            (f, t) if temp_units.contains(&f) && temp_units.contains(&t) => {
                ConversionUnit::Temperature
            }
            (f, t) if length_units.contains(&f) && length_units.contains(&t) => {
                ConversionUnit::Length
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
            ConversionUnit::Length => {
                let result = length::convert(self.value, &self.from, &self.to);
                println!("{result}{}", self.to);
            }

            ConversionUnit::Unknown => {
                let temp_units = temperature::get_units();
                let length_units = length::get_units();

                println!("Cannot convert, unknown unit. Available units,");
                println!("Temperature: {}", temp_units.join(", "));
                println!("Length: {}", length_units.join(", "));
            }
        }
    }
}
