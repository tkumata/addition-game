use std::io::Write;
use std::io::{self};
use std::sync::{mpsc::Receiver, mpsc::Sender, Arc, Mutex};
use std::thread;
use std::time::Duration;
use termion;
use termion::{color, style};

use crate::config::*;

pub struct TimerHandler {
    timer: Arc<Mutex<u64>>,
    timer_receiver: Receiver<()>,
    main_sender: Sender<()>,
    timeout: i32,
}

impl TimerHandler {
    pub fn new(
        timer: Arc<Mutex<u64>>,
        timer_receiver: Receiver<()>,
        main_sender: Sender<()>,
        timeout: i32,
    ) -> Self {
        Self {
            timer,
            timer_receiver,
            main_sender,
            timeout,
        }
    }

    pub fn start(self) {
        thread::spawn(move || {
            loop {
                if self.timer_receiver.try_recv().is_ok() {
                    return;
                }

                if let Ok(mut timer_value) = self.timer.try_lock() {
                    if *timer_value > self.timeout.try_into().unwrap() {
                        break;
                    }

                    // タイマー表示
                    print_timer((*timer_value).try_into().unwrap());
                    *timer_value += 1;
                }

                thread::sleep(Duration::from_secs(1));
            }

            // タイムアップ表示
            print_timeup();
            self.main_sender.send(()).unwrap();
        });
    }
}

// タイマー表示関数
fn print_timer(timer: i32) {
    print!("{}", termion::cursor::Save);
    print!("{}", termion::cursor::Goto(1, Y_STAT));
    print!("Time: {} sec", timer);
    print!("{}", termion::cursor::Restore);
    io::stdout().flush().unwrap();
}

// タイムアップ表示関数
fn print_timeup() {
    print!("{}", termion::cursor::Goto(1, Y_QUIT));
    print!("{}", termion::clear::AfterCursor);
    print!(
        "{}⏰Time up. Press ENTER key.↩️{}\r\n",
        color::Fg(color::Red),
        style::Reset
    );
}
