pub fn get_units() -> [String; 2] {
    [String::from("c"), String::from("f")]
}

fn to_base(degree: f32, unit: &str) -> f32 {
    match unit.to_lowercase().as_str() {
        "c" => degree,
        "f" => (degree - 32.0) * 5.0 / 9.0,
        _ => 0.0,
    }
}

fn from_base(base_degree: f32, target_unit: &str) -> f32 {
    match target_unit.to_lowercase().as_str() {
        "c" => base_degree,
        "f" => (base_degree * 9.0 / 5.0) + 32.0,
        _ => 0.0,
    }
}

pub fn convert(degree: f32, from: &str, to: &str) -> f32 {
    let base_degree = to_base(degree, from);
    let result = from_base(base_degree, to);
    (result * 10.0).round() / 10.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_celcius_to_fahrenheit() {
        assert_eq!(convert(32.0, "C", "F"), 89.6);
        assert_eq!(convert(25.2, "C", "F"), 77.4);
        assert_eq!(convert(16.4, "C", "F"), 61.5);
        assert_eq!(convert(-1.0, "C", "F"), 30.2);
    }

    #[test]
    fn convert_fahrenheit_to_celcius() {
        assert_eq!(convert(89.6, "F", "C"), 32.0);
        assert_eq!(convert(77.4, "F", "C"), 25.2);
        assert_eq!(convert(61.5, "F", "C"), 16.4);
        assert_eq!(convert(-10.0, "F", "C"), -23.3);
    }
}
