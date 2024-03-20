trait ClacArea {
    fn calc_area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl ClacArea for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
}

struct RightTriangle {
    width: f64,
    height: f64,
}

impl ClacArea for RightTriangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height * 0.5
    }
}

fn area<T: ClacArea>(x: &T) -> f64 {
    x.calc_area()
}

fn main() {
    let rect = Rectangle {
        width: 1.0,
        height: 2.0,
    };
    println!("{}", area(&rect));

    let tria = RightTriangle {
        width: 1.0,
        height: 2.0,
    };
    println!("{}", area(&tria));
}
