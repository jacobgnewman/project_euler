//Largest prime factor

//The prime factors of 13195 are 5, 7, 13 and 29.
//What is the largest prime factor of the number 600851475143 ?

fn main() {
        let mut target: u64 = 600851475143;
        let mut divisor = 3;
        loop{
            if target % divisor == 0 {
                target /= divisor;
            }
            divisor += 2;
            if divisor == target {
                break
            }
        }
        println!("{}", divisor)
}
