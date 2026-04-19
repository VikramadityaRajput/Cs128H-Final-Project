mod data;
mod math;

fn main() {
    println!("Checkpoint 1");

//Load Data
    match data::load_data("epl_results.csv") {
        Ok(matches) => {
            println!("Loaded {} matches.", matches.len());
            if let Some(m) = matches.first() {
                println!("Sample: {} vs {} -> {}", m.home_team, m.away_team, m.result);
            }
        }
        Err(e) => println!("Error loading data: {}", e),
    }
    let parent_gini = math::calculate_gini(&[10, 10], 20);
    let left_gini = math::calculate_gini(&[10, 0], 10);
    let right_gini = math::calculate_gini(&[0, 10], 10);
    
    let gain = math::calculate_information_gain(parent_gini, left_gini, 10, right_gini, 10);
    
    println!("Parent Gini: {:.4}", parent_gini);
    println!("Information Gain from perfect split: {:.4}", gain);
}