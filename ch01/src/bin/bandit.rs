use plot::Plot;

pub mod bandit {
    use rand::Rng;

    #[derive(Debug, Copy, Clone)]
    pub struct Bandit {
        rates: [f64; 10],
    }

    impl Bandit {
        pub fn new() -> Self {
            let rates = [rand::thread_rng().gen(); 10];
            Bandit { rates }
        }

        pub fn play(&self, arm: u32) -> u32 {
            let rate = self.rates[arm as usize];
            if rate > rand::thread_rng().gen() {
                1
            } else {
                0
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Agent {
        epsilon: f64,
        qualitys: [f64; 10],
        numbers: [u64; 10],
    }

    impl Agent {
        pub fn new(epsilon: f64) -> Self {
            let qualitys = [0.; 10];
            let numbers = [0; 10];
            Agent {
                epsilon,
                qualitys,
                numbers,
            }
        }

        pub fn update(&mut self, action: u32, reward: u32) {
            self.numbers[action as usize] += 1;
            self.qualitys[action as usize] += (reward as f64 - self.qualitys[action as usize])
                / self.numbers[action as usize] as f64;
        }

        pub fn get_action(&self) -> u32 {
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
}

pub mod plot {
    use plotters::prelude::*;

    pub trait Plot {
        fn plot(&self, name: &str, omittion: bool) -> Result<(), Box<dyn std::error::Error>>;
    }

    impl Plot for Vec<f32> {
        fn plot(&self, name: &str, omittion: bool) -> Result<(), Box<dyn std::error::Error>> {
            let size = self.len();
            let mut xs = vec![];
            let ys = self;
            for i in 0..size {
                xs.push(i as u32);
            }

            let image_width = 1080;
            let image_height = 720;
            let path = format!("{}{}{}", "./ch01/graph/", name, ".png");
            let root = BitMapBackend::new(&path, (image_width, image_height)).into_drawing_area();

            root.fill(&WHITE)?;

            let (y_min, y_max) = ys
                .iter()
                .fold((0.0 / 0.0, 0.0 / 0.0), |(m, n), v| (v.min(m), v.max(n)));

            let caption = name;
            let font = ("sans-serif", 20);

            let mut chart;

            if omittion {
                chart = ChartBuilder::on(&root)
                    .caption(caption, font.into_font())
                    .margin(10)
                    .x_label_area_size(16)
                    .y_label_area_size(42)
                    .build_cartesian_2d(*xs.first().unwrap()..*xs.last().unwrap(), y_min..y_max)?;
            } else {
                if y_min < 0. {
                    chart = ChartBuilder::on(&root)
                        .caption(caption, font.into_font())
                        .margin(10)
                        .x_label_area_size(16)
                        .y_label_area_size(42)
                        .build_cartesian_2d(
                            *xs.first().unwrap()..*xs.last().unwrap(),
                            y_min..y_max,
                        )?;
                } else {
                    chart = ChartBuilder::on(&root)
                        .caption(caption, font.into_font())
                        .margin(10)
                        .x_label_area_size(16)
                        .y_label_area_size(42)
                        .build_cartesian_2d(
                            *xs.first().unwrap()..*xs.last().unwrap(),
                            0f32..y_max,
                        )?;
                }
            }

            chart.configure_mesh().draw()?;

            let line_series =
                LineSeries::new(xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)), &RED);
            chart.draw_series(line_series)?;

            Ok(())
        }
    }

    impl Plot for Vec<u32> {
        fn plot(&self, name: &str, omittion: bool) -> Result<(), Box<dyn std::error::Error>> {
            let mut data = vec![];
            let size = self.len();
            for i in 0..size {
                data.push(self[i] as f32);
            }
            data.plot(name, omittion)
        }
    }
}

#[allow(dead_code)]
fn main() {
    let steps = 1000;
    let epsilon = 0.1;

    let bandit = bandit::Bandit::new();
    let mut agent = bandit::Agent::new(epsilon);
    let mut total_reward = 0;
    let mut total_rewards = vec![];
    let mut rates = vec![];

    for step in 0..steps {
        let action = agent.get_action();
        let reward = bandit.play(action);
        agent.update(action, reward);
        total_reward += reward;

        total_rewards.push(total_reward);
        rates.push(total_reward as f32 / (step + 1) as f32);
    }
    println!("{}", total_reward);

    // Plot reward change.
    let _ = total_rewards.plot("TotalRewards", true);
    // Plot rate change.
    let _ = rates.plot("Rates", false);
}
