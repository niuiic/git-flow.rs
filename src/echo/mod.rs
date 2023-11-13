#[cfg(test)]
mod test;

pub struct Echo {}

impl Echo {
    pub fn error(msg: &str) {
        println!("\x1B[31m\u{2718} {}\x1B[0m", msg);
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

    pub fn progress(msg: &str) {
        print!("\x1B[38;2;128;128;128m\u{23F3}{}\x1B[0m", msg);
    }
}
