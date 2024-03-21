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

trait CalcLength {
    fn calc_length(&self) -> f64;
}

struct Line {
    length: f64,
}

impl CalcLength for Line {
    fn calc_length(&self) -> f64 {
        self.length
    }
}

impl CalcLength for Rectangle {
    fn calc_length(&self) -> f64 {
        (self.width * self.height) * 2.0
    }
}

impl CalcLength for RightTriangle {
    fn calc_length(&self) -> f64 {
        self.width + self.height + (self.width.powi(2) + self.height.powi(2)).sqrt()
    }
}

fn length<T: CalcLength>(x: &T) -> f64 {
    x.calc_length()
}

fn main() {
    let rect = Rectangle {
        width: 1.0,
        height: 2.0,
    };
    println!("area = {}", area(&rect));
    println!("length = {}", length(&rect));

    let tria = RightTriangle {
        width: 1.0,
        height: 2.0,
    };
    println!("area = {}", area(&tria));
    println!("length = {}", length(&tria));

    let line = Line { length: 5.0 };
    // println!("area = {}", area(&line));
    println!("length = {}", length(&line));
}
