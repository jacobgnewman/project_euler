//Largest palindrome product

//A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

//Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    let max = 999;
    let min = 900;
    let mut check: Vec<i32> = Vec::new();
    for x in max - min..=max {
        for y in max - min..=max {
            check.push(x * y);
        }
    }
    check.sort();
    check.reverse();

    for i in check {
        if check_pal(i) {
            print!("{}", i);
            break;
        }
    }
}

fn check_pal(x: i32) -> bool {
    let s: String = x.to_string();
    let up_to = s.len() / 2;
    s.bytes().take(up_to).eq(s.bytes().rev().take(up_to))
}
