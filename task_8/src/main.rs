use std::{collections::HashMap, sync::{Arc, Mutex}, thread::spawn};

use dashmap::DashMap;

fn hash_map(){

    let map=Arc::new(Mutex::new(HashMap::new()));
    let mut threads=Vec::new();
    for i in 0..=10{
        let map=map.clone();
        let thread=spawn(move||{
            let mut guard=map.lock().expect("Error while get guard");

            guard.insert(i, "2")
        });
        threads.push(thread);
    }
    threads.into_iter().for_each(|thread|{
        thread.join().expect("Error while join thread");
    });

}
fn dash_map(){
    let map=Arc::new(DashMap::new());
    let mut threads=Vec::new();


    for i in 0..=10{
        let map=map.clone();
        let thread=spawn(move||{
            map.insert(i, "2")
        });
        threads.push(thread);
    }
    threads.into_iter().for_each(|thread|{
        thread.join().expect("Error while join thread");
    });
}


fn main() {
    hash_map();
    dash_map();
}
