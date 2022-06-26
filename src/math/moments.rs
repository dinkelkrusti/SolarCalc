pub fn mean(input: Vec<f64>) -> f64 {
    let len = input.len();

    let mut result = 0f64;
    for e in input {
        result += e;
    }

    let result = result/len;
    return(result);
}

pub fn standard_deviation(input: Vec<f64>) -> f64 {
    let mean = self::mean(input);
    let mut variance: f64 = 0f64;

    for e in input {
        variance += (e-mean).pow(2);
    }

    return variance.sqrt();
}