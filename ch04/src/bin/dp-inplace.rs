#[derive(Debug, Clone)]
struct Value {
    #[allow(dead_code)]
    state: String,
    value: f64,
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

    let mut count = 0;
    loop {
        let t = 0.5 * (-1. + 0.9 * v[0].value.clone()) + 0.5 * (1. + 0.9 * v[1].value.clone());
        let delta = (t - v[0].value).abs();
        v[0].value = t;

        let t = 0.5 * (0. + 0.9 * v[0].value.clone()) + 0.5 * (-1. + 0.9 * v[1].value.clone());
        let delta = delta.max((t - v[1].value).abs());
        v[1].value = t;

        count += 1;

        if delta < 0.0001 {
            println!("{:?}", v[0]);
            println!("{:?}", v[1]);
            println!("{}", count);
            break;
        }
    }
}
