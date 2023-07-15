use rand::Rng;
fn main() {
    let mut range = rand::thread_rng();
    let rnum: i32 = range.gen_range(1..=100);

    if rnum % 3 == 0 {
         println!("Fizz");

    } else if rnum % 5 == 0 {
        println!("Buzz");

    } else if rnum % 15 == 0 {
         println!("FizzBuzz");

    } else {
        println!("Nothing");
    }
println!("Представленное число {}", rnum);
    
}






