use rand::{Rng, rngs::ThreadRng};
use std::collections::VecDeque;

// 策略特征实现模块
// 定义博弈动作
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Cooperate,
    Betray,
}

// 策略特征
pub trait Strategy {
    fn decide(&mut self, actions_self: &[Action], actions_opponent: &[Action]) -> Action;
    fn reset(&mut self);
}

// 永久报复
pub struct Grudger {
    ever_betrayed: bool,
}
impl Strategy for Grudger {
    fn decide(&mut self, _: &[Action], actions_opponent: &[Action]) -> Action {
        if self.ever_betrayed {
            return Action::Betray;
        }

        if actions_opponent
            .iter()
            .find(|&&a| a == Action::Betray)
            .is_some()
        {
            self.ever_betrayed = true;
            Action::Betray
        } else {
            Action::Cooperate
        }
    }
    fn reset(&mut self) {
        self.ever_betrayed = false;
    }
}
// 概率型策略
pub struct Random {
    rng: rand::rngs::ThreadRng,
}
impl Strategy for Random {
    fn decide(&mut self, _: &[Action], _: &[Action]) -> Action {
        if rand::Rng::random_bool(&mut self.rng, 0.5) {
            Action::Cooperate
        } else {
            Action::Betray
        }
    }
    fn reset(&mut self) {
        self.rng = rand::rng();
    }
}

// 1. 以牙还牙
pub struct TitForTat {
    first_move: bool,
}

impl Strategy for TitForTat {
    fn decide(&mut self, _: &[Action], actions_opponent: &[Action]) -> Action {
        actions_opponent
            .last()
            .map(|x| *x)
            .unwrap_or(Action::Cooperate)
    }

    fn reset(&mut self) {
        self.first_move = true;
    }
}

// 2. 蒂德曼-基耶鲁齐
pub struct TidemanChieruzzi {
    consecutive_betrayals: usize,
    punishment_counter: usize,
}

impl Strategy for TidemanChieruzzi {
    fn decide(&mut self, _: &[Action], actions_opponent: &[Action]) -> Action {
        if actions_opponent.is_empty() {
            return Action::Cooperate;
        }

        let last_opponent = *actions_opponent.last().unwrap();
        if last_opponent == Action::Betray {
            self.consecutive_betrayals += 1;
            if self.consecutive_betrayals >= 2 {
                self.punishment_counter = self.consecutive_betrayals - 1;
                Action::Betray
            } else {
                Action::Betray
            }
        } else {
            self.consecutive_betrayals = 0;
            if self.punishment_counter > 0 {
                self.punishment_counter -= 1;
                Action::Betray
            } else {
                Action::Cooperate
            }
        }
    }

    fn reset(&mut self) {
        self.consecutive_betrayals = 0;
        self.punishment_counter = 0;
    }
}

// 3. 奈德格
pub struct Nydegger {
    prev_actions: VecDeque<Action>,
}

impl Nydegger {
    fn calculate_a(&self) -> u8 {
        let mut a = 0;
        if matches!(self.prev_actions.get(0), Some(Action::Betray)) {
            a += 4
        }
        if matches!(self.prev_actions.get(1), Some(Action::Betray)) {
            a += 2
        }
        if matches!(self.prev_actions.get(2), Some(Action::Betray)) {
            a += 1
        }
        a
    }
}

impl Strategy for Nydegger {
    fn decide(&mut self, _: &[Action], actions_opponent: &[Action]) -> Action {
        if actions_opponent.len() <= 3 {
            actions_opponent
                .last()
                .map(|x| *x)
                .unwrap_or(Action::Cooperate)
        } else {
            self.prev_actions
                .push_back(*actions_opponent.last().unwrap());
            if self.prev_actions.len() > 3 {
                self.prev_actions.pop_front();
            }

            match self.calculate_a() {
                0 | 1 | 6 | 7 => Action::Cooperate,
                _ => Action::Betray,
            }
        }
    }

    fn reset(&mut self) {
        self.prev_actions.clear();
    }
}

// 4. 格罗夫曼
pub struct Grofman {
    rng: ThreadRng,
}

impl Strategy for Grofman {
    fn decide(&mut self, actions_self: &[Action], actions_opponent: &[Action]) -> Action {
        if let (Some(last_self), Some(last_opponent)) =
            (actions_self.last(), actions_opponent.last())
        {
            if last_self == last_opponent {
                Action::Cooperate
            } else {
                if self.rng.random_bool(2.0 / 7.0) {
                    Action::Cooperate
                } else {
                    Action::Betray
                }
            }
        } else {
            Action::Cooperate
        }
    }

    fn reset(&mut self) {
        self.rng = rand::rng();
    }
}

// 5. 舒比克
pub struct Shubik {
    revenge_counter: usize,
    revenge_length: usize,
}

impl Strategy for Shubik {
    fn decide(&mut self, _: &[Action], actions_opponent: &[Action]) -> Action {
        if matches!(actions_opponent.last(), Some(Action::Betray)) {
            self.revenge_length += 1;
            self.revenge_counter = self.revenge_length;
        }

        if self.revenge_counter > 0 {
            self.revenge_counter -= 1;
            Action::Betray
        } else {
            Action::Cooperate
        }
    }

    fn reset(&mut self) {
        self.revenge_counter = 0;
        self.revenge_length = 1;
    }
}

// 6. 斯坦-拉波波特
pub struct SteinRapoport {
    rng: ThreadRng,
}

impl Strategy for SteinRapoport {
    fn decide(&mut self, _: &[Action], actions_opponent: &[Action]) -> Action {
        match actions_opponent.len() {
            0..=4 => Action::Cooperate,
            // s if s >= s - 2 => Action::Betray, // 这规则不知道怎么实现
            s => {
                if s % 15 == 0 {
                    let coop_count = actions_opponent
                        .iter()
                        .filter(|a| **a == Action::Cooperate)
                        .count();
                    let prob = coop_count as f64 / s as f64;
                    if (prob - 0.5).abs() < 0.2 {
                        // 如果对手看起来像随机
                        return Action::Betray;
                    }
                }
                *actions_opponent.last().unwrap()
            }
        }
    }

    fn reset(&mut self) {
        self.rng = rand::rng();
    }
}

// 8. 戴维斯
pub struct Davis {
    opponent_betrayed: bool,
}

impl Strategy for Davis {
    fn decide(&mut self, _: &[Action], actions_opponent: &[Action]) -> Action {
        let step = actions_opponent.len();
        if step <= 10 {
            Action::Cooperate
        } else {
            if self.opponent_betrayed {
                Action::Betray
            } else {
                self.opponent_betrayed = actions_opponent
                    .iter()
                    .find(|&&x| x == Action::Betray)
                    .is_some();
                if self.opponent_betrayed {
                    Action::Betray
                } else {
                    Action::Cooperate
                }
            }
        }
    }

    fn reset(&mut self) {
        self.opponent_betrayed = false;
    }
}

// 9. 格拉斯坎普
pub struct Graaskamp {
    random_detected: bool,
    last_actions: VecDeque<Action>,
}

impl Strategy for Graaskamp {
    fn decide(&mut self, _: &[Action], actions_opponent: &[Action]) -> Action {
        let step = actions_opponent.len();
        match step {
            0..=50 => actions_opponent
                .last()
                .map(|x| *x)
                .unwrap_or(Action::Cooperate),
            51 => Action::Betray,
            52..=56 => actions_opponent.last().map(|x| *x).unwrap(),
            _ => {
                if step >= 57 && !self.random_detected {
                    if self.last_actions.len() >= 10 {
                        let coop_count = self
                            .last_actions
                            .iter()
                            .filter(|&a| *a == Action::Cooperate)
                            .count();
                        let prob = coop_count as f64 / self.last_actions.len() as f64;
                        self.random_detected = (prob - 0.5).abs() < 0.1; // 检测对手是否是随机
                    }
                    self.last_actions
                        .push_back(*actions_opponent.last().unwrap());
                    if self.last_actions.len() > 10 {
                        self.last_actions.pop_front();
                    }
                }

                if self.random_detected {
                    Action::Betray
                } else {
                    *actions_opponent.last().unwrap()
                }
            }
        }
    }

    fn reset(&mut self) {
        self.random_detected = false;
        self.last_actions.clear();
    }
}

// 10. 唐宁
pub struct Downing {
    opp_coop_count: f64,
    opp_total: f64,
}

impl Strategy for Downing {
    fn decide(&mut self, _: &[Action], actions_opponent: &[Action]) -> Action {
        if let Some(&last) = actions_opponent.last() {
            self.opp_total += 1.0;
            if last == Action::Cooperate {
                self.opp_coop_count += 1.0;
            }

            let prob_coop = self.opp_coop_count / self.opp_total;
            if (5.0 * prob_coop + 1.0 * (1.0 - prob_coop))
                > (3.0 * prob_coop + 0 as f64 * (1.0 - prob_coop))
            {
                // 选择能最大化自身长期收益的行动.
                Action::Betray
            } else {
                Action::Cooperate
            }
        } else {
            Action::Cooperate
        }
    }

    fn reset(&mut self) {
        self.opp_coop_count = 0.0;
        self.opp_total = 0.0;
    }
}

// 11. 费尔德
pub struct Feld {
    consecutive_coop: usize,
    rng: ThreadRng,
}

impl Strategy for Feld {
    fn decide(&mut self, _: &[Action], actions_opponent: &[Action]) -> Action {
        if let Some(&last) = actions_opponent.last() {
            if last == Action::Cooperate {
                self.consecutive_coop += 1;
                let prob =
                    self.consecutive_coop as f64 / (10.0 + 2.0 * self.consecutive_coop as f64); // 概率持续下降直到 0.5
                if self.rng.random_bool(prob) {
                    Action::Cooperate
                } else {
                    Action::Betray
                }
            } else {
                self.consecutive_coop = 0;
                Action::Betray
            }
        } else {
            Action::Cooperate
        }
        .into()
    }

    fn reset(&mut self) {
        self.consecutive_coop = 0;
        self.rng = rand::rng();
    }
}

// 12. 乔斯
pub struct Joss {
    rng: ThreadRng,
}

impl Strategy for Joss {
    fn decide(&mut self, _: &[Action], actions_opponent: &[Action]) -> Action {
        if let Some(&last) = actions_opponent.last() {
            match last {
                Action::Betray => Action::Betray,
                Action::Cooperate => {
                    if self.rng.random_bool(0.9) {
                        Action::Cooperate
                    } else {
                        Action::Betray
                    }
                }
            }
        } else {
            Action::Cooperate
        }
    }

    fn reset(&mut self) {
        self.rng = rand::rng();
    }
}

// 13. 图洛克
pub struct Tullock {
    initial_phase: bool,
    coop_prob: f64,
}

impl Strategy for Tullock {
    fn decide(&mut self, _: &[Action], actions_opponent: &[Action]) -> Action {
        let step = actions_opponent.len();
        if step <= 11 {
            Action::Cooperate
        } else {
            if self.initial_phase {
                let coop_count = actions_opponent
                    .iter()
                    .take(10)
                    .filter(|&a| *a == Action::Cooperate)
                    .count();
                self.coop_prob = (coop_count as f64 / 10 as f64 * 0.9).max(0.0);
                self.initial_phase = false;
            }

            if rand::rng().random_bool(self.coop_prob) {
                Action::Cooperate
            } else {
                Action::Betray
            }
        }
    }

    fn reset(&mut self) {
        self.initial_phase = true;
        self.coop_prob = 1.0;
    }
}

// 14. 匿名策略, 该策略初始合作概率为 30%, 每 10 步更新一次, 根据对手行为调整合作概率, 但通常保持在 30%-70% 之间.
pub struct Anonymous {
    coop_prob: f64,
    rng: ThreadRng,
}

impl Strategy for Anonymous {
    fn decide(&mut self, _: &[Action], actions_opponent: &[Action]) -> Action {
        let step = actions_opponent.len();
        if step > 0 && step % 10 == 0 {
            let last_10 = actions_opponent.iter().rev().take(10);
            let opp_coop = last_10.filter(|&a| *a == Action::Cooperate).count();
            self.coop_prob =
                (self.coop_prob * 0.7 + (opp_coop as f64 / 10.0) * 0.3).clamp(0.3, 0.7);
        }

        if self.rng.random_bool(self.coop_prob) {
            Action::Cooperate
        } else {
            Action::Betray
        }
    }

    fn reset(&mut self) {
        self.coop_prob = 0.3;
        self.rng = rand::rng();
    }
}

// 15. 随机策略 (已实现)

// 更新策略工厂
pub fn create_strategy(id: usize) -> Box<dyn Strategy> {
    match id {
        1 => Box::new(TitForTat { first_move: true }),
        2 => Box::new(TidemanChieruzzi {
            consecutive_betrayals: 0,
            punishment_counter: 0,
        }),
        3 => Box::new(Nydegger {
            prev_actions: VecDeque::new(),
        }),
        4 => Box::new(Grofman { rng: rand::rng() }),
        5 => Box::new(Shubik {
            revenge_counter: 0,
            revenge_length: 1,
        }),
        6 => Box::new(SteinRapoport { rng: rand::rng() }),
        7 => Box::new(Grudger {
            ever_betrayed: false,
        }),
        8 => Box::new(Davis {
            opponent_betrayed: false,
        }),
        9 => Box::new(Graaskamp {
            random_detected: false,
            last_actions: VecDeque::new(),
        }),
        10 => Box::new(Downing {
            opp_coop_count: 0.0,
            opp_total: 0.0,
        }),
        11 => Box::new(Feld {
            consecutive_coop: 0,
            rng: rand::rng(),
        }),
        12 => Box::new(Joss { rng: rand::rng() }),
        13 => Box::new(Tullock {
            initial_phase: true,
            coop_prob: 1.0,
        }),
        14 => Box::new(Anonymous {
            coop_prob: 0.3,
            rng: rand::rng(),
        }),
        15 => Box::new(Random { rng: rand::rng() }),
        _ => panic!("Invalid strategy ID"),
    }
}
use csv::Writer;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

// 类型别名简化代码
type StrategyName = &'static str;

fn main() -> Result<(), Box<dyn Error>> {
    // 初始化数据存储
    let mut total_scores: HashMap<StrategyName, i32> = HashMap::new();
    let mut match_records = Vec::new();
    let round_per_simulation = 400; // 每次对局进行的回合.
    let simulation = 100; // 两个策略对局多少次.

    // 策略列表
    let strategies: [(usize, StrategyName); 15] = [
        (1, "TitForTat"), // 该策略首先合作, 随后每一轮模仿对手的上一轮选择, 即对手合作则合作, 对手背叛则背叛.
        (2, "TidemanChieruzzi"), // 该策略以合作开始, 采用以牙还牙策略. 然而, 当对手进行第二次连续背叛时, 增加惩罚力度, 每次对手连续背叛都会增加一次惩罚性背叛.
        (3, "Nydegger"), // 该策略在前三步使用以牙还牙策略, 之后根据前三步的结果计算一个值 A, 并根据 A 的特定值决定是否背叛.
        (4, "Grofman"), // 该策略在双方上一步选择相同时合作; 如果上一步选择不同, 则以 2/7 的概率合作.
        (5, "Shubik"),  // 该策略在对手背叛后进行报复, 最初背叛一次, 随后每次增加一次报复的长度.
        (6, "SteinRapoport"), // 该策略前四步合作, 之后采用以牙还牙策略, 每 15 步检查对手是否随机行动, 并在最后两步背叛.
        (7, "Grudger"),       // 该策略开始时合作, 但一旦对手背叛, 将永久背叛.
        (8, "Davis"),         // 该策略前 10 步合作, 之后如果对手曾经背叛过, 则永久背叛.
        (9, "Graaskamp"), // 该策略前 50 步采用以牙还牙策略, 第 51 步背叛, 接着再进行 5 步以牙还牙策略, 然后检测对手是否随机行动, 若是, 则持续背叛.
        (10, "Downing"), // 该策略根据对手合作或背叛的固定概率, 持续更新对对手行为的估计, 并选择能最大化自身长期收益的行动.
        (11, "Feld"), // 该策略采用以牙还牙策略, 但在对手合作后以逐渐降低的概率继续合作, 直到该概率降至 0.5.
        (12, "Joss"), // 该策略在对手合作后有 90% 的概率合作, 总是对对手的背叛进行背叛.
        (13, "Tullock"), // 该策略前 11 步合作, 之后根据对手在前 10 步的合作比例减少 10% 的合作概率.
        (14, "Anonymous"), // 该策略初始合作概率为 30%, 每 10 步更新一次, 根据对手行为调整合作概率, 但通常保持在 30%-70% 之间.
        (15, "Random"),    // 该策略以 50% 的概率随机选择合作或背叛.
    ];

    // 全排列对战模拟
    for (a_id, a_name) in strategies.iter() {
        for (b_id, b_name) in strategies.iter() {
            let mut a_total = 0;
            let mut b_total = 0;

            // 进行30次独立对战
            for _ in 0..simulation {
                let mut p1 = create_strategy(*a_id);
                let mut p2 = create_strategy(*b_id);
                let (score_a, score_b) = simulate(&mut *p1, &mut *p2, round_per_simulation);

                a_total += score_a;
                b_total += score_b;
            }

            // 记录对战结果
            match_records.push((
                *a_name,
                *b_name,
                a_total,
                b_total,
                a_total as f32 / simulation as f32, // 平均每次对战得分
                b_total as f32 / simulation as f32,
            ));

            // 更新总得分
            *total_scores.entry(a_name).or_insert(0) += a_total;
            *total_scores.entry(b_name).or_insert(0) += b_total;
        }
    }

    // 写入详细对战记录
    let mut match_wtr = Writer::from_writer(File::create("match_results.csv")?);
    match_wtr.write_record(&["PlayerA", "PlayerB", "TotalA", "TotalB", "AvgA", "AvgB"])?;

    for record in &match_records {
        match_wtr.write_record(&[
            record.0,
            record.1,
            &record.2.to_string(),
            &record.3.to_string(),
            &format!("{:.1}", record.4),
            &format!("{:.1}", record.5),
        ])?;
    }

    // 生成总得分排名
    let mut ranking: Vec<_> = total_scores.iter().collect();
    ranking.sort_by(|(_, a), (_, b)| b.cmp(a));

    // 写入总得分排名
    let mut rank_wtr = Writer::from_writer(File::create("ranking.csv")?);
    rank_wtr.write_record(&["Rank", "Strategy", "TotalScore", "AvgScorePerGame"])?;

    for (i, (name, score)) in ranking.iter().enumerate() {
        rank_wtr.write_record(&[
            &(i + 1).to_string(),
            **name,
            &score.to_string(),
            &format!("{:.1}", **score as f32 / simulation as f32),
        ])?;
    }

    // 打印结果摘要
    println!(
        "=== 最终排名 ({} 局 {} 轮) ===",
        simulation, round_per_simulation
    );
    for (i, (name, score)) in ranking.iter().enumerate() {
        println!(
            "{:2}. {:20} {:>8} {:>10.2}",
            i + 1,
            name,
            score,
            **score as f32 / simulation as f32
        );
    }

    println!("\n数据已保存到 match_results.csv 和 ranking.csv");
    Ok(())
}

// 模拟函数
fn simulate(a: &mut dyn Strategy, b: &mut dyn Strategy, rounds: usize) -> (i32, i32) {
    let mut actions_a = Vec::new();
    let mut actions_b = Vec::new();
    let (mut score_a, mut score_b) = (0, 0);

    a.reset();
    b.reset();

    for _ in 0..rounds {
        let action_a = a.decide(&actions_a, &actions_b);
        let action_b = b.decide(&actions_b, &actions_a);

        // 计算得分
        match (action_a, action_b) {
            (Action::Cooperate, Action::Cooperate) => {
                score_a += 3;
                score_b += 3;
            }
            (Action::Cooperate, Action::Betray) => {
                score_b += 5;
            }
            (Action::Betray, Action::Cooperate) => {
                score_a += 5;
            }
            (Action::Betray, Action::Betray) => {
                score_a += 1;
                score_b += 1;
            }
        }

        actions_a.push(action_a);
        actions_b.push(action_b);
    }

    (score_a, score_b)
}
