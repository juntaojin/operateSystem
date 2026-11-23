#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    let mut sum: i32 = 0;
    for i in 1..=100 {
        sum += i;
        if i % 10 == 0 {
            println!("sum(1..{}) = {}", i, sum);
        }
    }
    println!("Sum from 1 to 100 = {}", sum);
    sum
}