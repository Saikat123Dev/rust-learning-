fn main() {
    let name = String::from("hello");
    println!("{}", char_count(&name));
}

fn char_count(s:&str)-> usize{
    s.chars().count()
}