mod config;
mod presentation;

use config::Y_TARGET;
use presentation::timer_handler::TimerHandler;
use rand::{seq::SliceRandom, Rng};
use std::io;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use presentation::bgm_handler::*;
use presentation::ui_handler::*;

fn main() -> io::Result<()> {
    let args = UiHandler::parse_args();

    let (main_sender, main_receiver) = mpsc::channel();
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

    let target = generate_random_target(20, 40);
    print!("{}", termion::cursor::Goto(1, Y_TARGET));
    println!("ターゲット値: {}", target);

    let (components, incorrect) = generate_components_with_incorrect(target, 3);
    println!(
        "構成要素 (正しい要素と間違いの要素が混ざっています): {:?}",
        components
    );

    let selected_numbers = get_user_input(&components);

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
    main_receiver.try_recv().unwrap();

    println!(
        "正しい構成要素は: {:?}",
        components
            .iter()
            .filter(|&&x| !incorrect.contains(&x))
            .collect::<Vec<_>>()
    );

    Ok(())
}

// ランダムなターゲット値を生成
fn generate_random_target(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

// ターゲット値をランダムな複数の正の整数の和で構成し、間違いの値も混ぜる
fn generate_components_with_incorrect(target: i32, incorrect_count: usize) -> (Vec<i32>, Vec<i32>) {
    let mut rng = rand::thread_rng();
    let mut remaining = target;
    let mut correct_components = Vec::new();

    // 正しい構成要素を生成
    while remaining > 0 {
        let next = rng.gen_range(1..=remaining);
        correct_components.push(next);
        remaining -= next;
    }

    // 間違った値を生成
    let incorrect_components: Vec<i32> = (0..incorrect_count)
        .map(|_| rng.gen_range(1..=target))
        .collect();

    // 正しい構成要素と間違った値を結合し、ランダムに並べ替え
    let mut all_components = correct_components.clone();
    all_components.extend(&incorrect_components);
    all_components.shuffle(&mut rng);

    (all_components, incorrect_components)
}

// ユーザーからの入力を受け付け、候補リストに含まれるかをチェック
fn get_user_input(components: &[i32]) -> Vec<i32> {
    loop {
        println!("ターゲット値を構成する正しい要素をスペース区切りで入力してください:");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("入力の読み込みに失敗しました");

        let selected_numbers: Vec<i32> = user_input
            .trim()
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        // 候補に含まれていない数値があるかチェック
        if selected_numbers
            .iter()
            .all(|&num| components.contains(&num))
        {
            return selected_numbers;
        } else {
            println!(
                "入力した数値のうち、一部が候補リストに含まれていません。再入力してください。"
            );
        }
    }
}
