mod bandit;

use rand::Rng;

use crate::bandit::bandit::Agent;
use crate::bandit::plot::Plot;

#[derive(Debug, Clone, Copy)]
struct NonStatBandit {
    rates: [f64; 10],
}

impl NonStatBandit {
    fn new() -> Self {
        let mut rates: [f64; 10] = Default::default();
        for i in 0..10 as usize {
            rates[i] = rand::thread_rng().gen();
        }
        NonStatBandit { rates }
    }

    fn play(&mut self, arm: u32) -> u32 {
        self.rates[arm as usize] += 0.2 * (rand::thread_rng().gen::<f64>() - 0.5);
        let rate = self.rates[arm as usize];
        if rate > rand::thread_rng().gen() {
            1
        } else {
            0
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct AlphaAgent {
    epsilon: f64,
    qualitys: [f64; 10],
    alpha: f64,
}

impl AlphaAgent {
    fn new(epsilon: f64, alpha: f64) -> Self {
        let qualitys = [0.; 10];
        AlphaAgent {
            epsilon,
            qualitys,
            alpha,
        }
    }

    fn update(&mut self, action: u32, reward: u32) {
        self.qualitys[action as usize] +=
            (reward as f64 - self.qualitys[action as usize]) * self.alpha;
    }

    fn get_action(&self) -> u32 {
        if self.epsilon > rand::thread_rng().gen() {
            return rand::thread_rng().gen_range(1..10);
        }
        let mut result = 0;
        for i in 0..10 {
            if self.qualitys.iter().fold(0.0 / 0.0, |m, v| v.max(m)) == self.qualitys[i] {
                result = i as u32;
                break;
            }
        }
        return result;
    }
}

fn main() {
    let runs = 200;
    let steps = 1000;
    let epsilon = 0.1;
    let alpha = 0.8;

    let mut result = vec![];
    let mut sample_rates = vec![];

    for _ in 0..runs {
        let mut agent = Agent::new(epsilon);
        let mut bandit = NonStatBandit::new();
        let mut total_reward = 0;
        let mut rates = vec![];

        for step in 0..steps {
            let action = agent.get_action();
            let reward = bandit.play(action);
            agent.update(action, reward);
            total_reward += reward;

            rates.push(total_reward as f32 / (step + 1) as f32);
        }

        sample_rates.push(rates);
    }

    let mut avg_rates = vec![];

    for i in 0..sample_rates[0].len() {
        let mut element = 0.;
        for j in 0..200 {
            element += sample_rates[j][i];
        }
        element /= 200.;
        avg_rates.push(element);
    }

    result.push(avg_rates);

    let mut alpha_rates = vec![];
    for _ in 0..runs {
        let mut agent = AlphaAgent::new(epsilon, alpha);
        let mut bandit = NonStatBandit::new();
        let mut total_reward = 0;
        let mut rates = vec![];

        for step in 0..steps {
            let action = agent.get_action();
            let reward = bandit.play(action);
            agent.update(action, reward);
            total_reward += reward;

            rates.push(total_reward as f32 / (step + 1) as f32);
        }

        alpha_rates.push(rates);
    }

    let mut avg_rates = vec![];

    for i in 0..alpha_rates[0].len() {
        let mut element = 0.;
        for j in 0..200 {
            element += alpha_rates[j][i];
        }
        element /= 200.;
        avg_rates.push(element);
    }

    result.push(avg_rates);

    let _ = result[0].plot("SampleAverage", false);
    let _ = result[1].plot("AlphaConstUpdate", false);
}
