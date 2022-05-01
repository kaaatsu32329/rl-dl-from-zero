use rand::{self, Rng};

fn main() {
    // let mut rewards = vec![];
    let mut rng = rand::thread_rng();
    let mut quolity = 0f64;

    for n in 1..11 {
        let reward: f64 = rng.gen();
        // rewards.push(reward);
        // quolity = rewards.iter().sum();
        // quolity /= n as f64;
        quolity += (reward - quolity) / (n as f64);
        println!("Q = {}", quolity);
    }
}
