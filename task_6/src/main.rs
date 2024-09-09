use std::{sync::mpsc::channel, thread::{sleep, spawn}, time::{Duration, Instant}};

fn main() {
    let (sender,receiver)=channel::<()>();
    let start=Instant::now();
    let thread=spawn(||{

        receiver.into_iter().for_each(|_|{
            sleep(Duration::from_secs(5));
            println!("Message accepted")
        });


    });
    while start.elapsed()<Duration::from_secs(10)  {
        sleep(Duration::from_secs(1));
        match sender.send(()) {
            Ok(_)=>{
                println!("Message send successfully")
            }
            Err(err)=>{
                eprintln!("Error while send message:{}",err)
            }
            
        }
        
    }
    drop(sender);
    thread.join().expect("Error while join thread")
}
