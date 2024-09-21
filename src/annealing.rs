pub const INITIAL_LEARNING_RATE: f64 = 0.75;
pub const INITIAL_EXPLORATION_RATE: f64 = 0.2;

/// Function used for calculating the learning rate
pub fn learning_rate_function(initial_rate: f64, iteration: u32) -> f64 {
    // Currently uses a step decay
    let drop_rate:f64 = 0.9;
    let step_size: u32 = 20;
    initial_rate * drop_rate.powi((iteration/step_size) as i32)
}

/// Function used for calculating the exploration rate
pub fn exploration_rate_function(initial_rate: f64, iteration: u32) -> f64 {
    // Currently uses a step decay
    let drop_rate: f64 = 0.9;
    let step_size: u32 = 10;
    initial_rate * drop_rate.powi((iteration/step_size) as i32)
}