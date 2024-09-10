use std::{
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    thread::{sleep, spawn},
    time::Duration,
};

struct Conveyor {
    numbers: Vec<i64>,
    number_sender: SyncSender<i64>,
    number_receiver: Receiver<i64>,
    square_number_sender: SyncSender<i64>,
    square_number_receiver: Receiver<i64>,
}
impl Conveyor {
    fn new() -> Self {
        let numbers = (1..=100).collect::<Vec<i64>>();
        let (number_sender, number_receiver) = sync_channel::<i64>(50);
        let (square_number_sender, square_number_receiver) = sync_channel::<i64>(10);
        Self {
            numbers,
            number_receiver,
            number_sender,
            square_number_sender,
            square_number_receiver,
        }
    }
    fn run(self) {
        let number_sender = self.number_sender;
        let number_receiver = self.number_receiver;
        let square_number_sender = self.square_number_sender;
        let square_number_receiver = self.square_number_receiver;
        let numbers = self.numbers;
        //thread for send square of number
        let thread_recv_num = spawn(move || {
            number_receiver.into_iter().for_each(|num| {
                match square_number_sender.send(num * num) {
                    Ok(_) => {
                        println!("Square of number={} sent successfully", num)
                    }
                    Err(err) => {
                        eprintln!("Error while send square of num={}", err)
                    }
                }
            });
        });
        //thread of recv square of number
        let thread_recv_square_num = spawn(move || {
            square_number_receiver
                .into_iter()
                .for_each(|square_of_number| {
                    sleep(Duration::from_millis(100));
                    println!("Square:{}", square_of_number);
                })
        });
        //main thread send into channel numbers
        numbers
            .into_iter()
            .for_each(|num| match number_sender.send(num) {
                Ok(_) => {
                    sleep(Duration::from_millis(50));
                    println!("Number={} sent successfully", num)
                }
                Err(err) => {
                    eprintln!("Error while send num={}", err)
                }
            });
        drop(number_sender);
        thread_recv_num
            .join()
            .expect("Error while join thread_recv_num");
        thread_recv_square_num
            .join()
            .expect("Error while join thread_recv_square_num");
    }
}

fn main() {
    let conv = Conveyor::new();
    conv.run()
}
