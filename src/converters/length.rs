use std::collections::HashMap;

pub fn get_units() -> [String; 7] {
    [
        String::from("cm"),
        String::from("in"),
        String::from("ft"),
        String::from("yd"),
        String::from("m"),
        String::from("km"),
        String::from("mi"),
    ]
}

fn get_calculations() -> HashMap<String, f32> {
    let mut units = HashMap::new();
    units.insert(String::from("cm"), 1.0);
    units.insert(String::from("in"), 2.54);
    units.insert(String::from("ft"), 30.48);
    units.insert(String::from("yd"), 91.44);
    units.insert(String::from("m"), 100.0);
    units.insert(String::from("km"), 100000.0);
    units.insert(String::from("mi"), 160934.0);
    units
}

fn to_base(distance: f32, unit: &str) -> f32 {
    let units = get_calculations();
    units.get(unit.to_lowercase().as_str()).unwrap() * distance
}

fn from_base(base_distance: f32, target_unit: &str) -> f32 {
    let units = get_calculations();
    base_distance / units.get(target_unit.to_lowercase().as_str()).unwrap()
}

pub fn convert(distance: f32, from: &str, to: &str) -> f32 {
    let base_distance = to_base(distance, from);
    let result = from_base(base_distance, to);
    (result * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_from_cm() {
        assert_eq!(convert(100.0, "cm", "cm"), 100.0);
        assert_eq!(convert(100.0, "cm", "in"), 39.37);
        assert_eq!(convert(100.0, "cm", "ft"), 3.28);
        assert_eq!(convert(100.0, "cm", "yd"), 1.09);
        assert_eq!(convert(100.0, "cm", "m"), 1.0);
        assert_eq!(convert(10000.0, "cm", "km"), 0.1);
        assert_eq!(convert(10000.0, "cm", "mi"), 0.06);
    }

    #[test]
    fn convert_from_in() {
        assert_eq!(convert(39.37, "in", "cm"), 100.0);
        assert_eq!(convert(39.37, "in", "ft"), 3.28);
        assert_eq!(convert(39.37, "in", "yd"), 1.09);
        assert_eq!(convert(39.37, "in", "m"), 1.0);
    }

    #[test]
    fn convert_from_ft() {
        assert_eq!(convert(3.28, "ft", "cm"), 99.97);
        assert_eq!(convert(3.28, "ft", "in"), 39.36);
        assert_eq!(convert(3.28, "ft", "yd"), 1.09);
        assert_eq!(convert(3.28, "ft", "m"), 1.0);
    }

    #[test]
    fn convert_from_yd() {
        assert_eq!(convert(1.09, "yd", "cm"), 99.67);
        assert_eq!(convert(1.09, "yd", "in"), 39.24);
        assert_eq!(convert(1.09, "yd", "ft"), 3.27);
        assert_eq!(convert(1.09, "yd", "m"), 1.0);
    }

    #[test]
    fn convert_from_m() {
        assert_eq!(convert(1.0, "m", "cm"), 100.0);
        assert_eq!(convert(1.0, "m", "in"), 39.37);
        assert_eq!(convert(1.0, "m", "ft"), 3.28);
        assert_eq!(convert(1.0, "m", "yd"), 1.09);
    }

    #[test]
    fn convert_from_km() {
        assert_eq!(convert(0.1, "km", "cm"), 10000.0);
        assert_eq!(convert(0.1, "km", "in"), 3937.01);
        assert_eq!(convert(0.1, "km", "ft"), 328.08);
        assert_eq!(convert(0.1, "km", "yd"), 109.36);
    }

    #[test]
    fn convert_from_mi() {
        assert_eq!(convert(0.06, "mi", "cm"), 9656.04);
        assert_eq!(convert(0.06, "mi", "in"), 3801.59);
        assert_eq!(convert(0.06, "mi", "ft"), 316.8);
        assert_eq!(convert(0.06, "mi", "yd"), 105.6);
    }
}
