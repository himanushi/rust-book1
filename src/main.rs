fn main() {
    let mut st = String::from("Hello");

    st.push_str(", World!");
    println!("{}", st);
    println!("{}", &st[0..6]);
}
