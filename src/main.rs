fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5].into_iter().map(|x| x + 1).collect();
    println!("{:?}", v)
}
