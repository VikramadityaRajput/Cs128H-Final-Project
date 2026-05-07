use crate::data::MatchRecord;
use crate::math::{calculate_gini, calculate_information_gain};

#[derive(Debug)]
pub enum Node {
    Leaf(String),
    Internal {
        feature_index: usize,
        threshold: f64,
        left: Box<Node>,
        right: Box<Node>,
    },
}

pub struct DecisionTree {
    pub root: Option<Node>,
    pub max_depth: usize,
}

impl DecisionTree {
    pub fn new(max_depth: usize) -> Self {
        Self {
            root: None,
            max_depth,
        }
    }

    pub fn train(&mut self, data: &[MatchRecord]) {
        self.root = Some(self.build_tree(data, 0));
    }

    fn build_tree(&self, data: &[MatchRecord], depth: usize) -> Node {

        if depth >= self.max_depth || data.is_empty() || calculate_gini(data) == 0.0 {
            return Node::Leaf(self.get_majority_label(data));
        }

        let mut best_gain = 0.0;
        let mut best_feature = 0;
        let mut best_threshold = 0.0;
        let mut best_splits = (Vec::new(), Vec::new());

        for feature_index in 0..2 {
            let mut thresholds: Vec<f64> = data.iter().map(|m| self.get_feature_val(m, feature_index)).collect();
            thresholds.sort_by(|a, b| a.partial_cmp(b).unwrap());
            thresholds.dedup();

            for thresh in thresholds {
                let (left, right) = self.split_data(data, feature_index, thresh);
        
                if left.is_empty() || right.is_empty() {
                    continue;
                }

                let gain = calculate_information_gain(data, &left, &right);
                if gain > best_gain {
                    best_gain = gain;
                    best_feature = feature_index;
                    best_threshold = thresh;
                    best_splits = (left, right);
                }
            }
        }
        if best_gain == 0.0 {
            return Node::Leaf(self.get_majority_label(data));
        }

        Node::Internal {
            feature_index: best_feature,
            threshold: best_threshold,
            left: Box::new(self.build_tree(&best_splits.0, depth + 1)),
            right: Box::new(self.build_tree(&best_splits.1, depth + 1)),
        }
    }

    pub fn predict(&self, record: &MatchRecord) -> String {
        self.predict_node(self.root.as_ref().unwrap(), record)
    }

    fn predict_node(&self, node: &Node, record: &MatchRecord) -> String {
        match node {
            Node::Leaf(label) => label.clone(),
            Node::Internal { feature_index, threshold, left, right } => {
                let val = self.get_feature_val(record, *feature_index);
                if val > *threshold {
                    self.predict_node(left, record)
                } else {
                    self.predict_node(right, record)
                }
            }
        }
    }

    fn get_feature_val(&self, record: &MatchRecord, feature_index: usize) -> f64 {
        match feature_index {
            0 => record.half_time_home_goals as f64,
            1 => record.half_time_away_goals as f64,
            _ => 0.0,
        }
    }

    fn get_majority_label(&self, data: &[MatchRecord]) -> String {
        if data.is_empty() {
            return "D".to_string();
        }

        let mut h_count = 0;
        let mut a_count = 0;
        let mut d_count = 0;

        for m in data {
            match m.result.as_str() {
                "H" => h_count += 1,
                "A" => a_count += 1,
                _ => d_count += 1,
            }
        }

        if h_count >= a_count && h_count >= d_count {
            "H".to_string()
        } else if a_count >= d_count {
            "A".to_string()
        } else {
            "D".to_string()
        }
    }

    fn split_data(&self, data: &[MatchRecord], feature_index: usize, thresh: f64) -> (Vec<MatchRecord>, Vec<MatchRecord>) {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for m in data {
            if self.get_feature_val(m, feature_index) > thresh {
                left.push(m.clone());
            } else {
                right.push(m.clone());
            }
        }
        (left, right)
    }
}