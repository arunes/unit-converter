use std::collections::HashMap;

pub fn get_units() -> [String; 4] {
    [
        String::from("g"),
        String::from("oz"),
        String::from("lbs"),
        String::from("kg"),
    ]
}

fn get_calculations() -> HashMap<String, f32> {
    let mut units = HashMap::new();
    units.insert(String::from("g"), 1.0);
    units.insert(String::from("oz"), 28.3495);
    units.insert(String::from("lbs"), 453.592);
    units.insert(String::from("kg"), 1000.0);
    units
}

fn to_base(weight: f32, unit: &str) -> f32 {
    let units = get_calculations();
    units.get(unit.to_lowercase().as_str()).unwrap() * weight
}

fn from_base(base_weight: f32, target_unit: &str) -> f32 {
    let units = get_calculations();
    base_weight / units.get(target_unit.to_lowercase().as_str()).unwrap()
}

pub fn convert(weight: f32, from: &str, to: &str) -> f32 {
    let base_weight = to_base(weight, from);
    let result = from_base(base_weight, to);
    (result * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_from_g() {
        assert_eq!(convert(100.0, "g", "g"), 100.0);
        assert_eq!(convert(100.0, "g", "oz"), 3.53);
        assert_eq!(convert(100.0, "g", "lbs"), 0.22);
        assert_eq!(convert(100.0, "g", "kg"), 0.1);
    }

    #[test]
    fn convert_from_oz() {
        assert_eq!(convert(24.5, "oz", "g"), 694.56);
        assert_eq!(convert(24.5, "oz", "oz"), 24.5);
        assert_eq!(convert(24.5, "oz", "lbs"), 1.53);
        assert_eq!(convert(24.5, "oz", "kg"), 0.69);
    }

    #[test]
    fn convert_from_lbs() {
        assert_eq!(convert(5.2, "lbs", "g"), 2358.68);
        assert_eq!(convert(5.2, "lbs", "oz"), 83.20);
        assert_eq!(convert(5.2, "lbs", "lbs"), 5.2);
        assert_eq!(convert(5.2, "lbs", "kg"), 2.36);
    }

    #[test]
    fn convert_from_kg() {
        assert_eq!(convert(0.5, "kg", "g"), 500.0);
        assert_eq!(convert(0.5, "kg", "oz"), 17.64);
        assert_eq!(convert(0.5, "kg", "lbs"), 1.1);
        assert_eq!(convert(0.5, "kg", "kg"), 0.5);
    }
}
