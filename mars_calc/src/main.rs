fn main() {
    // If need to mutate a variable, use let mut instead of let
    // let mut mars_weight = calculate_weight_on_mars(100.0);
    // mars_weight = mars_weight * 1000.0;
    let mars_weight = calculate_weight_on_mars(100.0);
    println!("Weight on Mars: {} Kg", mars_weight);
}

fn calculate_weight_on_mars (weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}