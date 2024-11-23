use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    print!("Give me an integer\n");
    let input_str = iter.next().unwrap().unwrap();
    println!("Your numeber is {}", input_str);
    let n:u32 = input_str.parse().unwrap();
    let ret = is_prime(n);
    if ret{
        println!("{} is Prime", n);
    }else {
        println!("{} is NOT Prime", n);
    }
}

fn is_prime(n:u32) -> bool{
    if n == 1 {
        return false;
    } else if n == 2 {
        return true;
    }else if n % 2 == 0 {
        return false
    }

    let upper:u32 = (n as f32).sqrt().round() as u32;
    
    for i in (3..upper).step_by(2){
        if n % i == 0{
            return false;
        } 
    }

    return true;
}
