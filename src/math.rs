/// Information Gain = Gini(Parent) - [Weighted Average Gini(Children)]
pub fn calculate_information_gain(
    parent_gini: f64,
    left_gini: f64,
    left_count: usize,
    right_gini: f64,
    right_count: usize,
) -> f64 {
    let total = (left_count + right_count) as f64;
    let weighted_children_gini = (left_count as f64 / total * left_gini) 
                               + (right_count as f64 / total * right_gini);
    
    parent_gini - weighted_children_gini
}
pub fn calculate_gini(counts: &[usize], total: usize) -> f64 {
    if total == 0 { return 0.0; }
    
    let mut sum_sq = 0.0;
    for &count in counts {
        let p = count as f64 / total as f64;
        sum_sq += p * p;
    }
    
    1.0 - sum_sq
}