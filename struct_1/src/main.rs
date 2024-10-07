struct User{
    first_name:String,
    last_name:String,
    age:i32,
}

struct Rect{
    width:u32,
    height:u32,

}
impl Rect {
   fn area(&self)-> u32 {
      return self.width * self.height
   }
}


fn main() {
    let user = User{
        first_name: String::from("John"),
        last_name: String::from("Smith"),
        age: 20,
    };
    let rect = Rect{
        width:200,
        height:200
    };
    println!("{}",rect.area());
}
