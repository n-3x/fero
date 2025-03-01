use crate::engine::Value;

pub fn softmax(y: &Vec<Vec<Value>>) -> Vec<Vec<Value>> {
    y.iter()
        .map(|y_i| {
            let exp_yi = y_i.iter().map(|y_ij| y_ij.exp());
            let exp_sum = exp_yi.clone().sum::<Value>();
            exp_yi.map(|y_ij| y_ij / &exp_sum).collect()
        })
        .collect()
}
