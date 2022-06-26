pub fn mean(input: Vec<f64>) -> f64 {
    let len = input.len();

    let mut result = 0f64;
    for element in input {
        result += element;
    }

    let result = result/len;
    return(result);
}
