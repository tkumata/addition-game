use rand::{seq::SliceRandom, Rng};

pub struct Generate;

impl Generate {
    // ランダムなターゲット値を生成
    pub fn random_target(min: i32, max: i32) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }

    // ターゲット値をランダムな複数の正の整数の和で構成し、間違いの値も混ぜる
    pub fn components_with_incorrect(target: i32, incorrect_count: usize) -> (Vec<i32>, Vec<i32>) {
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
}
