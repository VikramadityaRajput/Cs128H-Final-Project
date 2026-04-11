# Cs128H-Final-Project
Group Name: Minecraft Decision Trees
Member: Vikramaditya Rajput - vrajput3

Project Introduction:
This project aims to build a custom Decision Tree Classifier from scratch in Rust to predict the outcomes of English Premier League soccer matches (Home Win, Away Win, or Draw). We will utilize historical match datasets from Kaggle, which include features such as team form, historical head-to-head records, shots on target, and referee assignments.We have chosen this project because it sits at the intersection of systems programming and statistical machine learning. Instead of using pre-built libraries like Python's scikit-learn, writing the algorithm from scratch in Rust allows us to deeply explore the data structures (trees) and the mathematical algorithms (Gini impurity or Entropy) that drive machine learning, while taking advantage of Rust's performance and memory safety.
Technical Overview:
The project will be built as a standalone Command Line Interface (CLI) application and consists of three main pipelines:Data Ingestion and Preprocessing: We will use the csv and serde crates to load the Kaggle dataset. The data must be cleaned and encoded (e.g., converting categorical string data like "Team Name" into numerical identifiers).The Decision Tree Algorithm: This is the core data structure. The tree will recursively partition the dataset. At each node, the algorithm must calculate the optimal feature and threshold to split the data. We will implement Gini Impurity to evaluate splits:
G = 1 - sum of i =1 to c of (p sub i)^2 where p-i is the probability of an item belonging to class i. The node will choose the split that maximizes Information Gain.
Evaluation Engine: The system will split the Kaggle data into training and testing sets (e.g., an 80/20 split), traverse the built tree with the unseen test data, and output accuracy metrics and a confusion matrix.

Checkpoint Roadmap
Checkpoint 1: * Successfully initialize the repository and project structure.

Implement the CSV parsing logic to load the Premier League dataset into structured Rust Vecs and structs.

Write the mathematical helper functions for calculating Gini Impurity and Information Gain.

Checkpoint 1.5 (personal checkpoint) * Implement the recursive Decision Tree builder logic using Rust's Box and enum types.

Successfully train a basic tree on a small subset of the data (overfitting is acceptable at this stage just to prove the tree builds correctly).

Checkpoint 2 (Final Polish): * Implement the train/test split logic and the evaluation module to measure prediction accuracy.

Add a CLI interface (using clap) so a user can pass a specific CSV file and hyper-parameters (like maximum tree depth) as arguments.

Clean up documentation and optimize the node-splitting loops for performance.

Possible Challenges
Handling Continuous vs. Categorical Variables: Decision trees handle numerical data (e.g., "Possession > 50%") differently than categorical data (e.g., "Referee == Michael Oliver"). Implementing splitting logic that smoothly handles both types in Rust's strong type system will be complex.

Rust Ownership in Recursive Trees: Building recursive data structures that mutate or share references to large datasets can easily anger the borrow checker. We will need to carefully manage ownership, likely cloning subsets of data indices rather than the data itself during the tree building phase.

Preventing Overfitting: We will need to implement a stopping condition (like max_depth or min_samples_split) to ensure the tree doesn't just memorize the Kaggle dataset.

References
Kaggle Dataset: English Premier League Results (e.g., English Premier League Data).
