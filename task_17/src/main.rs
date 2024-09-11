use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex}, thread::spawn};

fn atomic()->usize{
    let atomic=Arc::new(AtomicUsize::new(0));
    let num_threads=50;
    let mut threads=Vec::with_capacity(50);

    for _ in 1..=num_threads  {
        let atomic=atomic.clone();
        let thread=spawn(move ||{
                for _ in 0..80{
                    atomic.fetch_add(1, Ordering::SeqCst);
                }
        }); 
        threads.push(thread)       
    }

    threads.into_iter().for_each(|thread|{
        thread.join().expect("Error while join thread")
    });
    atomic.load(Ordering::SeqCst)
}

fn mutex()->i32{
    let arc_mutex=Arc::new(Mutex::new(0));

    let num_threads=50;
    let mut threads=Vec::with_capacity(50);

    for _ in 1..=num_threads  {
        let data=arc_mutex.clone();
        let thread=spawn(move ||{
                for _ in 0..80{
                    match data.lock() {
                        Ok(mut guard)=>{
                            *guard+=1;
                        }
                        Err(err)=>{
                            eprintln!("Error while get lock:{}",err);
                        }
                    }
                }
        }); 
        threads.push(thread)       
    }

    threads.into_iter().for_each(|thread|{
        thread.join().expect("Error while join thread")
    });
    let result=*arc_mutex.lock().expect("Error while get guard");
    result
}



fn main() {
    assert_eq!(4000,atomic());
    assert_eq!(4000,mutex())
}
