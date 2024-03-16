struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    fn age_incr(&self, incr: u8) -> u8 {
        self.age + incr
    }

    fn age_incr_replace(&mut self, incr: u8) {
        self.age += incr;
    }
}

fn main() {
    let mut taro = Person::new(String::from("taro"), 10);

    let age_plus1 = taro.age_incr(1);
    println!("{}", age_plus1);

    taro.age_incr_replace(10);
    println!("{}", taro.age);

    let age_incr = Person::age_incr(&taro, 1);
    println!("{}", age_incr);
}
