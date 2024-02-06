use std::io::{self, Write};

use tokio::{
    sync::oneshot,
    time::{sleep, Duration},
};

#[cfg(test)]
mod test;

pub struct Echo {}

impl Echo {
    pub fn error<T: Into<String>>(msg: T) {
        eprintln!("\x1B[31m\u{2718} {}\x1B[0m", msg.into());
    }

    pub fn info<T: Into<String>>(msg: T) {
        let icon = "\u{1F6C8}";
        println!("\x1B[34m{} {}\x1B[0m", icon, msg.into());
    }

    pub fn warning<T: Into<String>>(msg: T) {
        let icon = "\u{26A0}";
        println!("\x1B[33m{} {}\x1B[0m", icon, msg.into());
    }

    pub fn success<T: Into<String>>(msg: T) {
        println!("\x1B[32m\u{2714} {}\x1B[0m", msg.into());
    }

    pub fn progress<T: Into<String>>(msg: T) -> Box<dyn FnOnce(bool, &str)> {
        let (tx, mut rx) = oneshot::channel();

        let msg = msg.into();
        let spinners = vec!["\u{280B}", "\u{2819}", "\u{2839}", "\u{2838}", "\u{283C}"];

        tokio::spawn(async move {
            loop {
                for spinner in &spinners {
                    if rx.try_recv().is_ok() {
                        return;
                    }

                    print!("\r\x1B[38;2;128;128;128m{} {}\x1B[0m", spinner, msg);
                    io::stdout().flush().unwrap();

                    sleep(Duration::from_millis(100)).await;
                }
            }
        });

        let finish = move |success: bool, msg: &str| {
            if success {
                print!("\r");
                Echo::success(msg);
            } else {
                println!();
                Echo::error(msg);
            }
            tx.send(()).unwrap();
        };
        Box::new(finish)
    }
}
