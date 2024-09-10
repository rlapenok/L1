



fn set<'a>(num:i64,position:usize,bit_value:bool)->Result<i64,&'a str>{
    if position>64{
        return Err("Position out of bounds. Valid range 0-63")
    }
    if bit_value{
        return Ok(num | (1<<position))
    }
    Ok(num & !(1<<position))
}


fn main() {
    let num=140 as i64;
    let position=47;
    let bit_value=true;
    let result=set(num, position, bit_value);
    println!("{result:?}");
}
