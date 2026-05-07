use crate::data::MatchRecord;
use std::collections::HashMap;

pub fn calculate_gini(data: &[MatchRecord]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    
    let mut counts = HashMap::new();
    for m in data {
        *counts.entry(&m.result).or_insert(0) += 1;
    }
    
    let mut impurity = 1.0;
    let total = data.len() as f64;
    for count in counts.values() {
        let prob = (*count as f64) / total;
        impurity -= prob * prob;
    }
    
    impurity
}

pub fn calculate_information_gain(parent: &[MatchRecord], left: &[MatchRecord], right: &[MatchRecord]) -> f64 {
    let weight_left = left.len() as f64 / parent.len() as f64;
    let weight_right = right.len() as f64 / parent.len() as f64;
    
    calculate_gini(parent) - (weight_left * calculate_gini(left)) - (weight_right * calculate_gini(right))
}