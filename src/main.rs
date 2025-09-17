use std::{io};
use num_bigint::BigInt;
use num_traits::{Zero,One};
fn fibonachi(num: u32)->BigInt{
    if num == 0 {
        return Zero::zero();
    }

    let mut a:BigInt = Zero::zero();
    let mut b:BigInt = One::one();
    for _ in 2..num{
        let  c = a + &b ;
        a = b;
        b = c ;
    };
    b
}
fn main(){
    println!("enter number for calc fibonachi");
    let mut input = String::new() ;
    io::stdin().read_line(&mut input).expect("e");
    let num:u32 =  input.trim().parse().expect("msg");
    let result  = fibonachi(num);
    println!("ficonachi({}): {}",num,result);
}