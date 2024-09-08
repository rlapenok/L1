use std::{cmp::min, sync::Arc, thread::spawn};


fn job(numbers:&[i64]){

    for i in numbers{
        println!("Square num:{} is:{}",i,i*i)
    }

}

fn main() {
let numbers=Arc::new((0..=800001).collect::<Vec<i64>>());
let workers=1000;
let mut workers_handle=Vec::with_capacity(workers);
let part_for_worker=numbers.len()/workers;
    for worker_num in 0..workers{
        let numbers=numbers.clone();
        let start_index=worker_num*part_for_worker;
        let end_index=min(start_index+part_for_worker, numbers.len());
        let worker=spawn(move ||{
                   
           job( &numbers[start_index..end_index])
        });
        workers_handle.push(worker)
    }
    for worker in workers_handle  {
        if let Err(err) =worker.join()  {
            eprintln!("Worker dropped with error:{:?}",err)
        }else {
        println!("Worker dropped successfully")
            
        }
        
    }

}
