use rand::Rng;

// 1. Activation Functions
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn relu(x: f64) -> f64 {
    x.max(0.0)
}


// 2. Forward Propagation (Single Layer)
fn forward_propagate(inputs: &Vec<f64>, weights: &Vec<Vec<f64>>, biases: &Vec<f64>, activation: fn(f64) -> f64) -> Vec<f64> {
    let mut outputs = Vec::new();
    for j in 0..weights.len() { // for each neuron in current layer
        let mut weighted_sum = biases[j];
         for i in 0..inputs.len() { // for each input to neuron
           weighted_sum += inputs[i] * weights[j][i];
        }
        outputs.push(activation(weighted_sum));
    }
    outputs
}


// 3. Loss Function (Mean Squared Error - Simplified)
fn mse_loss(predictions: &Vec<f64>, targets: &Vec<f64>) -> f64 {
    if predictions.len() != targets.len() {
        panic!("Prediction and target vectors must have the same length");
    }

    let mut sum_squared_errors = 0.0;
    for i in 0..predictions.len() {
        let error = predictions[i] - targets[i];
        sum_squared_errors += error * error;
    }
    sum_squared_errors / (predictions.len() as f64)
}

// 4. Simplified Gradient Calculation (Illustrative)
// Assume we have already performed forward pass and have the predictions.
// This is VERY basic. In real training you'd backpropagate error.
fn calculate_gradients(inputs: &Vec<f64>, weights: &Vec<Vec<f64>>, predictions: &Vec<f64>, targets: &Vec<f64>, learning_rate: f64) -> (Vec<Vec<f64>>, Vec<f64>) {
    // Simple gradient calculations are used only for demonstration purpose!
    // Real gradient calculations involve back propagation
    let num_neurons = weights.len();
    let num_inputs = inputs.len();
    let mut weight_gradients = vec![vec![0.0; num_inputs]; num_neurons];
    let mut bias_gradients = vec![0.0; num_neurons];
    
    for j in 0..num_neurons {
        let error = predictions[j] - targets[j]; // Simplified loss contribution
        bias_gradients[j] = error * learning_rate;
        for i in 0..num_inputs {
           weight_gradients[j][i] = error * inputs[i] * learning_rate;
        }
    }
    (weight_gradients, bias_gradients)
}


// Helper function for creating random weights and biases
fn create_random_matrix(rows: usize, cols: usize) -> Vec<Vec<f64>> {
     let mut rng = rand::thread_rng();
    let mut matrix = Vec::with_capacity(rows);
    for _ in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for _ in 0..cols {
           row.push(rng.gen_range(-1.0..1.0)); 
        }
         matrix.push(row);
    }
    matrix
}

fn create_random_vector(size: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(-1.0..1.0)).collect()
}



fn main() {
    // --- Example usage ---
    let inputs = vec![1.0, 2.0, 3.0];
    let num_neurons = 2;
    let num_inputs = inputs.len();

    // Initialize random weights and biases for a single layer
     let mut weights = create_random_matrix(num_neurons, num_inputs); 
    let mut biases = create_random_vector(num_neurons);
     
    let targets = vec![0.0, 1.0]; // Desired output for the inputs
    let learning_rate = 0.01;

    println!("Initial Weights: {:?}", weights);
    println!("Initial Biases: {:?}", biases);

    // Forward propagation 
    let mut predictions = forward_propagate(&inputs, &weights, &biases, sigmoid);
    println!("Initial Predictions: {:?}", predictions);

    //Calculate loss
     let loss = mse_loss(&predictions, &targets);
     println!("Initial Loss: {}", loss);
    
     // Simplified Update using gradient information
     let (weight_gradients, bias_gradients) = calculate_gradients(&inputs, &weights, &predictions, &targets, learning_rate);

    // Update weights and biases
     for j in 0..num_neurons {
        for i in 0..num_inputs {
           weights[j][i] -= weight_gradients[j][i];
        }
        biases[j] -= bias_gradients[j];
    }
     println!("Updated Weights: {:?}", weights);
     println!("Updated Biases: {:?}", biases);

     // Forward propagation after update
     predictions = forward_propagate(&inputs, &weights, &biases, sigmoid);
      println!("Predictions After Update: {:?}", predictions);

     //Calculate loss again after update
     let loss_after = mse_loss(&predictions, &targets);
      println!("Loss After Update: {}", loss_after);
}
