use std::str::FromStr;

use num_bigint::BigInt;

fn sum(num1:&BigInt,num2:&BigInt)->BigInt{
    num1+num2
}
fn sub(num1:&BigInt,num2:&BigInt)->BigInt{
    num1-num2
}

fn mul(num1:&BigInt,num2:&BigInt)->BigInt{
    num1*num2
}
fn div(num1:&BigInt,num2:&BigInt)->BigInt{
    num1/num2
}


fn main() {
    let num1=BigInt::from_str("340282366920938463463374607431768211454").expect("Error while create BigInt");
    let num2=BigInt::from_str("340282366920938463463374607431768211451").expect("Error while create BigInt");
    println!("{}",sum(&num1, &num2));
    println!("{}",sub(&num1, &num2));
    println!("{}",mul(&num1, &num2));
    println!("{}",div(&num1, &num2));
}
