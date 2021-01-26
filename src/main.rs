extern crate chrono;
extern crate timer;
use std::io::{self, Write};
use std::process::Command;
use std::sync::mpsc::channel;
fn main() {
    let timer = timer::Timer::new();
    let (tx, rx) = channel();
    let _g = timer.schedule_repeating(chrono::Duration::seconds(1), move || {
        let _ = tx.send(());
    });

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || args[1].len() < 2 {
        println!("need count down duration, example: 10s, 2m");
        println!("Usage: ./tick 2m");
        return;
    }
    let ut = &args[1];
    let len = ut.len();
    let ext = &ut[len - 1..len];
    let count_str = &ut[0..len - 1];
    let mut count = count_str.parse::<u32>().unwrap();
    let mul = match ext {
        "s" => 1,
        "m" => 60,
        _ => panic!("unknow time format"),
    };
    count = count * mul;
    println!("start {} count down", ut);
    loop {
        let _ = rx.recv().unwrap();
        print!("--> {}\r", pretty_time(count));
        io::stdout().flush().unwrap();
        if count <= 0 {
            break;
        }
        count -= 1;
    }
    //    println!("This code has been executed after {}", ut);

    Command::new("say")
        .arg("Your job should be done")
        .output()
        .unwrap();
}

fn pretty_time(secs: u32) -> String {
    let m = secs / 60;
    let s = secs % 60;

    format!("{:02}:{:02}", m, s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pretty_time() {
        assert_eq!(pretty_time(3), "00:03");
        assert_eq!(pretty_time(60), "01:00");
        assert_eq!(pretty_time(70), "01:10");
    }
}
