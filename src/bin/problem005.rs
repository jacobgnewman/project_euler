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