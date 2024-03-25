fn main() {
    let add_one = |x: i32| -> i32 { x + 1 };
    let v: Vec<i32> = vec![1, 2, 3, 4, 5].into_iter().map(add_one).collect();
    println!("{:?}", v)
}
