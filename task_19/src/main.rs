fn main() {
    let data="WB работа — моя цель".split_whitespace().rev().collect::<Vec<&str>>().join(" ");
    println!("{}",data)
    
}
