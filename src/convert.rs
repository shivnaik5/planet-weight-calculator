pub fn calculate_weight_on_planet(planet: String, weight: f32, gravity: f32) {
    let weight_on_planet = convert_weight(weight, gravity);
    let weight_in_lbs = convert_kg_to_lbs(weight_on_planet);

    println!("The weight on {} is {} kg, or {} lbs!", planet, weight_on_planet, weight_in_lbs);
}

fn convert_weight(weight: f32, conversion: f32) -> f32 {
    (weight / 9.81) * conversion
}

fn convert_kg_to_lbs(weight: f32) -> f32 {
    weight * 2.20462
} 