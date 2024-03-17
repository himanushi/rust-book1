fn func_ex_unwrap(x: i32) -> Option<i32> {
    if x >= 0 {
        Some(x)
    } else {
        None
    }
}

fn main() {
    let rr1 = func_ex_unwrap(10).unwrap();
    println!("rr1: {}", rr1);

    let rr2 = func_ex_unwrap(-5).unwrap();
    println!("rr2: {}", rr2);
}
