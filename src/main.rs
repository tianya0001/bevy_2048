fn main() 
{
    let x = 5 + 5;

    println!("10进制 x = {}", x);
    println!("2进制  x = {:b}", x);
    println!("8进制  x = {:o}", x);
    println!("16进制 x = {:x}", x);
    println!("16进制 x = {:X}", x);

    println!("{:0>8}", x);
}
