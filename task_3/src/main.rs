use std::{
    cmp::min,
    sync::{
        mpsc::{sync_channel, SyncSender},
        Arc,
    },
    thread::spawn,
};

fn job(numbers: &[i64], sender: SyncSender<i64>) {
    let mut sum = 0;
    numbers.iter().for_each(|num| sum +=num * num);
    if let Err(err) = sender.send(sum) {
        eprintln!("Error while send sum of square:{}", err)
    }
}

fn main() {
    let numbers = Arc::new((0..=3).collect::<Vec<i64>>());
    let workers = 3;
    let mut handles=Vec::with_capacity(workers);
   let part=(numbers.len()+workers-1)/workers;
    let (sender, receiver) = sync_channel::<i64>(workers);
    for i in 0..workers {
        let sender = sender.clone();
        let numbers = numbers.clone();
        let start = i * part;
        let end = min(start + part, numbers.len());
        let thread = spawn(move || job(&numbers[start..end], sender));
        handles.push(thread)
    }
    let mut result=0;

  for _ in 0..workers{
    match receiver.recv() {
        Ok(data)=>{
            result+=data;
        }
        Err(err)=>{
            eprintln!("Error while receive data from thread:{}",err)
        }
    }
  } 
  println!("Result:{}",result);
}
