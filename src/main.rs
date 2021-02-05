mod cli;
mod json;
mod convert;

fn main() {
    let args = cli::get_args();

    let weight = args.weight;
    let planet = format_planet_arg(args.planet.trim().to_string());

    let file_name: String = String::from("src/planet_gravity.json");
    let planets: serde_json::Value = json::read_json(file_name).unwrap();

    let planet_name = &planet;
    let available_planet = &planets[planet_name];

    if available_planet.is_null() {
        println!("What solar system are you in?!");
        std::process::exit(1);
    }

    let planet_gravity: f32 = (available_planet.to_string()).parse::<f32>().unwrap();

    convert::calculate_weight_on_planet(planet, weight, planet_gravity);
}

fn format_planet_arg(planet_arg: String) -> String {
    let mut formatted_planet_name = planet_arg.chars();
    match formatted_planet_name.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + formatted_planet_name.as_str(),
    }
}
