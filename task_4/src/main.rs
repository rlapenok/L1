use std::{
    sync::{
        mpsc::{channel, sync_channel, Receiver, Sender, SyncSender},
        Arc, Mutex,
    },
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

type Task = Box<dyn FnOnce() + Send + 'static>;

#[allow(dead_code)]
struct Worker {
    id: usize,
    handle: JoinHandle<()>,
    access_sender: Arc<SyncSender<usize>>,
}
impl Worker {
    fn new(
        id: usize,
        receiver: Receiver<Task>,
        access_sender: Arc<SyncSender<usize>>,
        sleep_for_worker: u64,
    ) -> Self {
        //wrapp receiver tasks
        let receiver = Arc::new(Mutex::new(receiver));
        let access_sender_for_thread = access_sender.clone();
        let handle = spawn(move || {
            //sending that the work is ready to receive tasks
            access_sender_for_thread.send(id).unwrap();
            println!("Worker № {} started successfully", id);
            loop {
                match receiver.lock() {
                    Ok(ok) => match ok.recv() {
                        Ok(task) => {
                            task();
                            println!("Worker № {} completed the task", id);
                            sleep(Duration::from_secs(sleep_for_worker));
                            access_sender_for_thread.send(id).unwrap();
                        }
                        Err(err) => {
                            eprintln!("Worker № {} stop with error:{}", id, err);
                        }
                    },
                    Err(err) => {
                        eprintln!("Worker № {} start,posion error:{}", id, err);
                    }
                }
            }
        });
        Self {
            id,
            handle,
            access_sender,
        }
    }
}
#[allow(dead_code)]
struct Pool {
    task_senders: Vec<Sender<Task>>,
    pool: Vec<Worker>,
    access_queue: Receiver<usize>,
}

impl Pool {
    fn new(size: usize) -> Self {
        //create vec of join handle
        let mut pool = Vec::with_capacity(size);
        //create vec for send task sender for each worker
        let mut task_senders = Vec::with_capacity(size);
        //create channel for send and recive access of worker
        let (access_sender, access_receiver) = sync_channel::<usize>(size);
        //create workers
        for i in 1..=size {
            //create channel for task sender and receiver
            let (task_sender, task_receiver) = channel::<Task>();
            let access_sender = Arc::new(access_sender.clone());
            //create worker
            let worker = Worker::new(i, task_receiver, access_sender.clone(), i as u64);
            //push worker task sender into pool
            task_senders.push(task_sender);
            //push joinhandler of worker
            pool.push(worker)
        }
        Self {
            pool,
            task_senders,
            access_queue: access_receiver,
        }
    }
    //spawn new task
    fn spawn<T>(&self, task: T)
    where
        T: FnOnce() + Send + 'static,
    {
        //get id access worker
        match self.access_queue.recv() {
            Ok(worker_id) => {
                //get task sender of worker
                let task_sender = &self.task_senders[worker_id - 1];
                let task = Box::new(task);
                match task_sender.send(task) {
                    Ok(_) => {
                        println!("Task for Worker № {} sent successfully", worker_id)
                    }
                    Err(err) => {
                        eprintln!("Error while send task fro Worker №{} is {}", worker_id, err);
                    }
                }
            }
            Err(err) => {
                println!("Error while receive worker access{}", err);
            }
        }
    }
    fn run(&self) {
        loop {
            let task = move || {};
            self.spawn(task)
        }
    }
}

fn main() {
    let pool = Pool::new(5);
    pool.run();
}
