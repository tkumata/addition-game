use std::io;

pub struct UserInput;

impl UserInput {
    // ユーザーからの入力を受け付け、候補リストに含まれるかをチェック
    pub fn get_user_input(components: &[i32]) -> Vec<i32> {
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
}
