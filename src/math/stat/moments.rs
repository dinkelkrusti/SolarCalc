pub fn mean(input: Vec<f64>) -> f64 {
    let len = input.len() as f64;

    let mut result = 0f64;
    for e in input {
        result += e;
    }

    let result = result/len;
    return result;
}

pub fn standard_deviation(input: Vec<f64>) -> f64 {
    let mean= self::mean(input.clone());
    let mut variance: f64 = 0f64;

    for e in input {
        variance += (e-mean).powf(2.0);
    }

    return variance.sqrt();
}