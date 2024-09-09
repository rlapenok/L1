use std::{cmp::min, sync::Arc, thread::spawn};

fn job(numbers: &[i64]) {
    for i in numbers {
        println!("Square num:{} is:{}", i, i * i)
    }
}

fn main() {
    let numbers = Arc::new((0..=7).collect::<Vec<i64>>());
    let workers = 3;
    let mut handles=Vec::with_capacity(workers);
   let part=(numbers.len()+workers-1)/workers;
 for i in 0..workers{
    let numbers=numbers.clone();
    let start=i*part;
    let end =min(start+part, numbers.len());
    let thread=spawn(move||{
        job(&numbers[start..end])
    });
    handles.push(thread)
    
 }
    for handle in handles {
        if let Err(err) = handle.join() {
            eprintln!("Worker dropped with error:{:?}", err)
        } else {
            println!("Worker dropped successfully")
        }
    }
}
