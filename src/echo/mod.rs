use std::io::{self, Write};

use tokio::{
    sync::oneshot,
    time::{sleep, Duration},
};

#[cfg(test)]
mod test;

pub struct Echo {}

impl Echo {
    pub fn error(msg: &str) {
        eprintln!("\x1B[31m\u{2718} {}\x1B[0m", msg);
    }

    pub fn info(msg: &str) {
        println!("\x1B[34m\u{1F6C8} {}\x1B[0m", msg);
    }

    pub fn warn(msg: &str) {
        println!("\x1B[33m\u{26A0} {}\x1B[0m", msg);
    }

    pub fn success(msg: &str) {
        println!("\x1B[32m\u{2714} {}\x1B[0m", msg);
    }

    pub fn progress(msg: &str) -> Box<dyn FnOnce()> {
        let (tx, mut rx) = oneshot::channel();

        let msg = msg.to_string();
        let spinners = vec!["\u{280B}", "\u{2819}", "\u{2839}", "\u{2838}", "\u{283C}"];

        tokio::spawn(async move {
            'task: loop {
                for spinner in &spinners {
                    if let Ok(_) = rx.try_recv() {
                        break 'task;
                    }

                    print!("\r\x1B[38;2;128;128;128m{} {}\x1B[0m", spinner, &msg);
                    io::stdout().flush().unwrap();

                    sleep(Duration::from_millis(100)).await;
                }
            }
        });

        let stop = move || {
            tx.send(()).unwrap();
        };

        Box::new(stop)
    }
}
