use proconio::input;
use rand::{prelude::SmallRng, Rng, SeedableRng};
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

const N: usize = 1000; // 注文の数（1000 で固定）

// 注文（Order）を管理する構造体
#[derive(Debug, Clone, Copy, PartialEq)]
struct Order {
    rest: Target,  // レストラン
    house: Target, // 家
    id: usize,     // 注文の index (0-indexed)
}

// 目的地（Target）を管理する構造体
#[derive(Debug, Clone, Copy, PartialEq)]
struct Target {
    point: Point,
    id: usize,        // 注文の index (0-indexed)
    kind: TargetKind, // レストラン or 家
}

// 目的地（Target）の種類を表す Enum
#[derive(Debug, Clone, Copy, PartialEq)]
enum TargetKind {
    Rest,  // レストラン
    House, // 家
}

// 座標の情報を管理する構造体
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32, // x 座標
    y: i32, // y 座標
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

// 座標間の距離を求める
fn get_dist(p0: Point, p1: Point) -> u32 {
    ((p0.x - p1.x).abs() + (p0.y - p1.y).abs()) as u32
}

// レストラン [0,49] を順番に回った後、家 [0,49] を順番に回る
// (処理する注文のVec, 訪れる座標のVec) を返す
fn make_0_49_simple_ans(orders: &Vec<Order>) -> (Vec<usize>, Vec<Point>) {
    let ans_ids = (0..50).collect::<Vec<_>>();
    let ans_points = vec![Point::new(400, 400)]
        .into_iter()
        .chain(orders[..50].iter().map(|o| o.rest.point.clone()))
        .chain(orders[..50].iter().map(|o| o.house.point.clone()))
        .chain(vec![Point::new(400, 400)])
        .collect::<Vec<_>>();
    (ans_ids, ans_points)
}

// 注文 [0,49] を「目的地のうち一番近いところに行く」貪欲法で解く
// (処理する注文のVec, 訪れる座標のVec) を返す
fn make_0_49_greedy_ans(orders: &Vec<Order>) -> (Vec<usize>, Vec<Point>) {
    let orders50 = orders[..50].to_vec();
    let ans_ids = (0..50).collect::<Vec<_>>();
    let mut ans_points = vec![]; // 行った座標をここに格納していく

    // (400, 400) からスタート
    let mut curr_point = Point::new(400, 400);
    ans_points.push(curr_point);

    // レストラン 50 件を貪欲法で回る
    let mut visited = HashSet::new();
    for _ in 0..50 {
        let next_rest = orders50
            .iter()
            .filter(|o| !visited.contains(&o.id))
            .min_by_key(|o| get_dist(o.rest.point, curr_point))
            .unwrap()
            .rest;
        visited.insert(next_rest.id);
        ans_points.push(next_rest.point);
        curr_point = next_rest.point;
    }

    // 家 50 件を貪欲法で回る
    let mut visited = HashSet::new();
    for _ in 0..50 {
        let next_house = orders50
            .iter()
            .filter(|o| !visited.contains(&o.id))
            .min_by_key(|o| get_dist(o.house.point, curr_point))
            .unwrap()
            .house;
        visited.insert(next_house.id);
        ans_points.push(next_house.point);
        curr_point = next_house.point;
    }

    // (400, 400) に戻る
    ans_points.push(Point::new(400, 400));

    // (処理する注文のVec, 訪れる座標のVec) を返す
    (ans_ids, ans_points)
}

// スコア計算用の関数（スコアが合う以外はちゃんとテストしてないです）
// 必要に応じて、ブラックボックスとして使用してください。
fn calc_score(orders: &Vec<Order>, ans_order_ids: &Vec<usize>, ans_points: &Vec<Point>) -> u32 {
    if ans_points.is_empty() {
        eprintln!("ans_points is empty.");
        return 0;
    }
    let home = Point::new(400, 400);
    if ans_points[0] != home || *ans_points.last().unwrap() != home {
        eprintln!("start and goal must be (400, 400).");
        return 0;
    }
    let mut pending_ids = ans_order_ids.iter().collect::<HashSet<_>>();
    if ans_order_ids.len() != pending_ids.len() {
        eprintln!("some order ids are duplicated.");
        return 0;
    }
    let mut picked_ids = HashSet::new();
    let mut point_to_kind_ids = HashMap::new();
    for order in orders {
        point_to_kind_ids
            .entry(order.rest.point)
            .or_insert(vec![])
            .push((TargetKind::Rest, order.id));
        point_to_kind_ids
            .entry(order.house.point)
            .or_insert(vec![])
            .push((TargetKind::House, order.id));
    }
    for point in ans_points {
        if !point_to_kind_ids.contains_key(point) {
            continue;
        }
        let kind_ids = &point_to_kind_ids[point];
        for &(kind, id) in kind_ids {
            if kind == TargetKind::Rest && pending_ids.contains(&id) {
                pending_ids.remove(&id);
                picked_ids.insert(id);
            } else if kind == TargetKind::House && picked_ids.contains(&id) {
                picked_ids.remove(&id);
            }
        }
    }
    if !pending_ids.is_empty() || !picked_ids.is_empty() {
        eprintln!("some orders have not been done.");
        return 0;
    }
    let dist = (0..ans_points.len() - 1)
        .map(|i| get_dist(ans_points[i], ans_points[i + 1]))
        .sum::<u32>();

    if ans_order_ids.len() != 50 {
        eprintln!("warning: ans_order_ids len is not 50.");
    }

    // score = (10**8)/(1000+dist)
    1e8 as u32 / (1000 + dist)
}

fn main() {
    let time_start = Instant::now();

    input! {
        abcd_v: [(i32,i32,i32,i32); N], // Vec<(i32,i32,i32,i32)>
    }

    // ----- 入力から Order の一覧を生成 -----
    // 1000件の注文（Order）が順番に Vec に入っています
    let orders = abcd_v
        .iter()
        .enumerate()
        .map(|(id, &abcd)| {
            let (a, b, c, d) = abcd;
            Order {
                rest: Target {
                    point: Point::new(a, b),
                    id,
                    kind: TargetKind::Rest,
                },
                house: Target {
                    point: Point::new(c, d),
                    id,
                    kind: TargetKind::House,
                },
                id,
            }
        })
        .collect::<Vec<_>>();

    // ----- レストラン [0,49] を順番に回った後、家 [0,49] を順番に回る で解く -----
    // let (ans_order_ids, ans_points) = make_0_49_simple_ans(&orders);

    // ----- 注文 [0,49] を「目的地のうち一番近いところに行く」貪欲法で解く -----
    let (ans_order_ids, ans_points) = make_0_49_greedy_ans(&orders);

    // ----- 時間制限いっぱいまで解の改善をする -----
    // （毎回時間計測すると遅いので、通常は適切なタイミングで時間計測を入れます）
    // ランダムで 2 つの地点を入れ替えてみて、スコアが改善されるか確認するなど...
    const TIME_LIMIT: u128 = 1900; // ms
    let mut rng = SmallRng::seed_from_u64(445); // 乱数生成器
    while time_start.elapsed().as_millis() < TIME_LIMIT {
        // 乱数で色々やる例
        // let rand_ind = rng.gen_range(0, 50); // [0,50)
        // let rand_percent = rng.gen::<f64>(); // [0.0, 1.0]
    }

    // ----- スコアの確認@ローカル -----
    let score = calc_score(&orders, &ans_order_ids, &ans_points);
    eprintln!("score: {}", score);

    // ----- 答えの出力 -----
    // <処理する注文の個数: m> <処理する注文のid_1> <処理する注文のid_2> ... <処理する注文のid_m>
    // <訪問する地点の個数: n> <訪問する地点のx座標_1> <訪問する地点のy座標_1> ... <訪問する地点のx座標_n> <訪問する地点のy座標_n>
    print!("{} ", ans_order_ids.len());
    for &order_id in &ans_order_ids {
        print!("{} ", order_id + 1);
    }
    println!();
    print!("{} ", ans_points.len());
    for &point in &ans_points {
        print!("{} {} ", point.x, point.y);
    }
    println!();
}
