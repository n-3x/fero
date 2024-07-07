use micrograd::{
    engine::{Activation, Value},
    nn::Neuron,
};

fn main() {
    let n = Neuron::new(2, Some(Activation::ReLU)).name_params();

    let x = vec![Value::new(-2.0), Value::new(1.0)];
    let x = n.name_inputs(x);

    println!("{}\n", n);

    let y = n.forward(&x);
    println!("Forward pass:\n{}", y.tree());

    y.backward();
    println!("Backward pass:\n{}", y.tree());
}
