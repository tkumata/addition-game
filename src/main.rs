mod application;
mod config;
mod presentation;

use config::Y_TARGET;
use std::io;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use application::usecases::generate::Generate;
use application::usecases::user_input::UserInput;
use presentation::bgm_handler::*;
use presentation::timer_handler::TimerHandler;
use presentation::ui_handler::*;

fn main() -> io::Result<()> {
    let args = UiHandler::parse_args();

    let (main_sender, _main_receiver) = mpsc::channel();
    let (snd_sender, snd_receiver) = mpsc::channel();
    let (_timer_sender, timer_receiver) = mpsc::channel();

    if args.sound {
        let bgm_handler = BgmHandler::new(snd_receiver);
        bgm_handler.start();
    }

    print!("{}", termion::clear::All);

    let timer = Arc::new(Mutex::new(0));
    let timer_clone = Arc::clone(&timer);
    let timer_handler = TimerHandler::new(timer_clone, timer_receiver, main_sender, args.timeout);
    timer_handler.start();

    let target = Generate::random_target(20, 40);
    print!("{}", termion::cursor::Goto(1, Y_TARGET));
    println!("ターゲット値: {}", target);

    let (components, incorrect) = Generate::components_with_incorrect(target, 3);
    println!(
        "構成要素 (正しい要素と間違いの要素が混ざっています): {:?}",
        components
    );

    let selected_numbers = UserInput::get_user_input(&components);

    let user_sum: i32 = selected_numbers.iter().sum();

    if user_sum == target {
        println!("正解！合計はターゲット値と一致しています。");
    } else {
        println!(
            "不正解です。合計: {} (正しいターゲット値: {})",
            user_sum, target
        );
    }

    snd_sender.send(()).unwrap();
    // main_receiver.try_recv().unwrap();

    println!(
        "正しい構成要素は: {:?}",
        components
            .iter()
            .filter(|&&x| !incorrect.contains(&x))
            .collect::<Vec<_>>()
    );

    Ok(())
}
