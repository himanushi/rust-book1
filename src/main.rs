#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let taro = Person {
        name: String::from("taro"),
        age: 10,
    };
    println!("{:?}", taro);
}
