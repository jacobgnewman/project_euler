//Smallest multiple

//2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn main(){
    let mut start = 2520;
    loop{
        start +=20;
        let mut pass = true;
        for i in 3..21 {
            if start % i != 0{
                pass = false;
            }
        }
        if pass {
            break
        }
    }
    println!("{}",start)
}
fn crt(max:f32){
    let primes = primes(max as i32);
    let mut N = 1;
    let mut i = 1;
    let mut check = true;
    let limit = max.sqrt();
    while check {

    }
}
fn primes(n: i32)-> Vec<i32> {

    return Vec::new()

}
