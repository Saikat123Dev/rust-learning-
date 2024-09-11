fn main() {
    println!("{}", fib(4));
}


fn fib(n:i32)->i32{
   let mut first = 0;
   let mut second = 1;
   if n==0{
       return 0;
   }
   if n==1{
    return 1;
   }
   for _ in 0..n-1{
       let temp = second;
       second = second + first;
       first = temp;
   }
   return second
    
}