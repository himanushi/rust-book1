struct GenEx<T> {
    value: T,
}

impl<T> GenEx<T> {
    fn return_input(self) -> T {
        self.value
    }
}

fn main() {
    let x1 = GenEx { value: 10 };
    let x2 = GenEx {
        value: "Hello".to_string(),
    };
    let x3 = GenEx::<f64> { value: 2.0 };

    println!("x1: {}", x1.return_input());
    println!("x2: {}", x2.return_input());
    println!("x3: {}", x3.return_input());
}
