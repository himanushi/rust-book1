trait IAbs<T, S> {
    fn iabs(self) -> S;
}

impl IAbs<i32, u32> for i32 {
    fn iabs(self) -> u32 {
        if self >= 0 {
            self as u32
        } else {
            -self as u32
        }
    }
}

fn main() {
    println!("{}", 1.iabs());
    println!("{}", (-1).iabs());
}
