use super::*;

/// Returns array of weights that are distributed exponentially
pub fn exp_weights(amount: usize) -> Vec<u32> {
    use rand::distributions::{Distribution, Exp};
    let exp = Exp::new(1.0);
    let weights_f: Vec<f64> = exp.sample_iter(&mut rand::thread_rng())
        .take(amount)
        .collect();
    let weight_sum = weights_f.iter().fold(0.0, |acc, x| acc + x);
    weights_f
        .iter()
        .map(|x| ((x / weight_sum) * ((<u32>::max_value()) / 2) as f64).floor() as u32)
        .collect()
}
