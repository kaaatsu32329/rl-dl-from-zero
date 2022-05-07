mod bandit;

use crate::bandit::bandit::{Agent, Bandit};
use crate::bandit::plot::Plot;

fn main() {
    let runs = 200;
    let steps = 1000;
    let epsilon = 0.1;
    let mut all_rates = vec![];

    for _ in 0..runs {
        let bandit = Bandit::new();
        let mut agent = Agent::new(epsilon);
        let mut total_reward = 0;
        let mut total_rewards = vec![];
        let mut rates = vec![];

        for step in 0..steps {
            let action = agent.get_action();
            let reward = bandit.clone().play(action);
            agent.update(action, reward);
            total_reward += reward;

            total_rewards.push(total_reward);
            rates.push(total_reward as f32 / (step + 1) as f32);
        }
        println!("{}", total_reward);
        all_rates.push(rates);
    }

    let mut avg_rates = vec![];

    for i in 0..all_rates[0].len() {
        let mut element = 0.;
        for j in 0..200 {
            element += all_rates[j][i];
        }
        element /= 200.;
        avg_rates.push(element);
    }

    // Plot average rates.
    let _ = avg_rates.plot("AverageRate", false);
}
