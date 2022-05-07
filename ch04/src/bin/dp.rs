#[derive(Debug, Clone)]
struct Value {
    #[allow(dead_code)]
    state: String,
    value: f64,
}

trait Max<T> {
    fn max(self, object_v: T) -> T;
}

impl Max<f64> for f64 {
    fn max(self, object_v: f64) -> f64 {
        if self > object_v {
            return self;
        } else {
            return object_v;
        }
    }
}

fn main() {
    let mut v: [Value; 2] = [
        Value {
            state: "L1".to_string(),
            value: 0f64,
        },
        Value {
            state: "L2".to_string(),
            value: 0f64,
        },
    ];
    let mut new_v = v.clone();

    let mut count = 0;
    loop {
        new_v[0].value =
            0.5 * (-1. + 0.9 * v[0].value.clone()) + 0.5 * (1. + 0.9 * v[1].value.clone());
        new_v[1].value =
            0.5 * (0. + 0.9 * v[0].value.clone()) + 0.5 * (-1. + 0.9 * v[1].value.clone());

        let delta = (new_v[0].value - v[0].value).abs();
        let delta = delta.max((new_v[1].value - v[1].value).abs());
        v = new_v.clone();

        count += 1;

        if delta < 0.0001 {
            println!("{:?}", v);
            println!("{}", count);
            break;
        }
    }
}
