use std::io::{stdin, stdout, Write};

fn main() {

    let mut global_buff=String::new();
    loop {
        let mut buff=String::new();
        print!("Unput data:");
        stdout().flush().unwrap();
        stdin().read_line(&mut buff).expect("Error while read from stdin");
        buff.trim().split_whitespace().for_each(|data|{
            if !global_buff.contains(data){
                global_buff+=&((data.to_owned()+" "))
            }else {
                println!("This line already exists")
            }
        });
    }
}
