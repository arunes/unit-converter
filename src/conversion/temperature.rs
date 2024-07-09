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

pub fn convert(degree: f32, from: &String, to: &str) -> f32 {
    let base_degree = to_base(degree, from);
    from_base(base_degree, to)
}
