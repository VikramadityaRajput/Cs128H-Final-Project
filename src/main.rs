use clap::Parser;

mod data;
mod math;
mod models;

use models::DecisionTree;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    #[arg(short, long)]
    file: String,

    #[arg(short, long, default_value_t = 3)]
    depth: usize,
}

fn main() {
    let args = Args::parse();

    let all_matches = data::load_data(&args.file).expect("Failed to load CSV");

    let split_index = (all_matches.len() as f64 * 0.8) as usize;
    let (train_data, test_data) = all_matches.split_at(split_index);

    println!("Loaded {} matches.", all_matches.len());
    println!("Training on {} matches | Testing on {} matches", train_data.len(), test_data.len());

    let mut tree = DecisionTree::new(args.depth);
    tree.train(train_data);
    println!("Tree trained successfully to depth {}!", args.depth);

    let mut correct = 0;
    for record in test_data {
        let prediction = tree.predict(record);
        if prediction == record.result {
            correct += 1;
        }
    }

    let accuracy = (correct as f64 / test_data.len() as f64) * 100.0;
    println!("Model Accuracy: {:.2}%", accuracy);
}