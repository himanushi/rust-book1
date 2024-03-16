#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Parents<'a, 'b> {
    father: &'a Person,
    mother: &'b Person,
}

impl<'a, 'b> Parents<'a, 'b> {
    fn new(father: &'a Person, mother: &'b Person) -> Parents<'a, 'b> {
        Parents { father, mother }
    }
}

fn main() {
    let taro = Person {
        name: "Taro".to_string(),
        age: 50,
    };
    let hanako = Person {
        name: "Hanako".to_string(),
        age: 48,
    };

    let sato = Parents::new(&taro, &hanako);

    println!("{:?}", sato);
    println!("{:?}", hanako);
}
