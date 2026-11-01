use std::sync::Mutex;
use tauri::State;
use rand::seq::SliceRandom; // シャッフルに使う機能
use rand::rng;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
enum card_color {
    Blue,
    Green,
    Red,
    Yellow,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
enum card_number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Skip,
    Reverse,
    Draw,
    Wild,
    WildDraw,
    ShuffleWild
}

struct GameConfig {
    people: u32,
    play_times: u32
}

/* cardはcolorとcard_numberからなる配列*/
struct Deck {
    cards: Vec<(card_color, card_number)>
}

impl Deck {
    fn new() -> Self {
        // 既定の順番通りにカードを格納
        let mut grid = Vec::with_capacity(109);
        for r in 0..16 {
            let mut card_number = card_number::Zero;
            let mut card_color = card_color::Blue;
            match r {
                0 => card_number = card_number::Zero,
                1 => card_number = card_number::One,
                2 => card_number = card_number::Two,
                3 => card_number = card_number::Three,
                4 => card_number = card_number::Four,
                5 => card_number = card_number::Five,
                6 => card_number = card_number::Six,
                7 => card_number = card_number::Seven,
                8 => card_number = card_number::Eight,
                9 => card_number = card_number::Nine,
                10 => card_number = card_number::Draw,
                11 => card_number = card_number::Reverse,
                12 => card_number = card_number::Skip,
                13 => card_number = card_number::Wild,
                14 => card_number = card_number::WildDraw,
                15 => card_number = card_number::ShuffleWild,
                _ => card_number = card_number::Zero,
            }
            match r {
                0 =>
                    /* 0 は 各色1枚ずつ*/ 
                    for i in 0..4 {
                        match i {
                            0 => card_color = card_color::Blue,
                            1 => card_color = card_color::Green,
                            2 => card_color = card_color::Red,
                            3 => card_color = card_color::Yellow,
                            _ => card_color = card_color::None,
                        }
                        grid.push((card_color, card_number));
                    },
                1..=9 => 
                    /* 1~9 は各色2枚ずつ */
                    for i in 0..2 {
                        for j in 0..4 {
                            match j {
                                0 => card_color = card_color::Blue,
                                1 => card_color = card_color::Green,
                                2 => card_color = card_color::Red,
                                3 => card_color = card_color::Yellow,
                                _ => card_color = card_color::None,
                            }
                            grid.push((card_color, card_number));
                        }
                    },
                10..=12 => 
                    /* Draw, Reverse, Skip は各色2枚ずつ */
                    for i in 0..2 {
                        for j in 0..4 {
                            match j {
                                0 => card_color = card_color::Blue,
                                1 => card_color = card_color::Green,
                                2 => card_color = card_color::Red,
                                3 => card_color = card_color::Yellow,
                                _ => card_color = card_color::None,
                            }
                            grid.push((card_color, card_number));
                        }
                    },
                13..=14 => 
                    /* Wild, WildDraw は4枚ずつ */
                    for i in 0..4 {
                        card_color = card_color::None;
                        grid.push((card_color, card_number));
                    }
                15 => {
                    /* ShuffleWild は1枚 */ 
                    card_color = card_color::None;
                    grid.push((card_color, card_number));
                }
                _ => println!("Unexpected r"),
            }
        }
        Deck { cards: grid }
    }

    fn show_cards(&self) {
        println!("[Debug] --- デッキのカード一覧 (合計: {}枚) ---", self.cards.len());
        for (i, (color, number)) in self.cards.iter().enumerate() {
            println!("[Debug] {:3}番目: 色 = {:?}, 種類 = {:?}", i + 1, color, number);
        }
    }

    fn shuffle(&mut self) {
        println!("[Debug] カードをシャッフルしています");
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    // 一番上のカードを取得する関数
    fn get_top_card(&self) -> (card_color, card_number) {
        self.cards[0]
    }

    fn draw_card(&mut self) -> (card_color, card_number) {
        self.cards.remove(0)
    }
}

struct CpuCards {
    cards: Vec<Vec<(card_color, card_number)>>
}

impl CpuCards {
    fn new() -> Self {
        CpuCards { cards: Vec::new() }
    }
}

struct PlayerCards {
    cards: Vec<(card_color, card_number)>
}

impl PlayerCards {
    fn new() -> Self {
        PlayerCards { cards: Vec::new() }
    }

    fn get_cards(&self) -> Vec<(card_color, card_number)> {
        self.cards.clone()
    }
}

// 盤面の一番上のカード
struct StageCard {
    card: (card_color, card_number)
}

impl StageCard {
    fn new() -> Self {
        StageCard { card: (card_color::None, card_number::Zero) }
    }

    fn register(card_color: card_color, card_number: card_number) -> Self {
        StageCard { card: (card_color, card_number) }
    }

    fn get_stage_card(&self) -> (card_color, card_number) {
        self.card
    }
}

#[tauri::command]
fn setup(
    people: u32,
    play_times: u32,
    state_game_config: State<Mutex<GameConfig>>,
    state_deck: State<Mutex<Deck>>,
    state_cpu_cards:State<Mutex<CpuCards>>,
    state_player_cards: State<Mutex<PlayerCards>>,
    state_stage_card: State<Mutex<StageCard>>
){
    // state.lock().unwrap() で中身を取り出す（合言葉のようなものです）
    let mut data = state_game_config.lock().unwrap();
    data.people = people;
    data.play_times = play_times;

    println!("保存完了: 人数 = {}, 回数 = {}", data.people, data.play_times);

    // シャッフルしていないデッキを作成
    let mut deck = state_deck.lock().unwrap();
    *deck = Deck::new(); // デッキを初期化（deck変数はこの時点では金庫になっているため、*deck参照することで、deckの中に入っているものを書き換えられる）
    loop {
        deck.shuffle();
        // 最初の一枚がWildDraw4の場合のみやり直し発生
        // 他のWild系の場合、最初のプレイヤーが色を決める
        let top_card = deck.get_top_card();
        if top_card.1 == card_number::WildDraw{
            continue;
        } else{
            break;
        }
    }

    // 一番上のカードをステージのカードに
    let mut stage_card = state_stage_card.lock().unwrap();
    let will_be_stage_card = deck.draw_card();
    *stage_card = StageCard::register(will_be_stage_card.0, will_be_stage_card.1);

    // シャッフル完了
    let top_card = deck.get_top_card();
    println!("{:?}", top_card.0);
    println!("{:?}", top_card.1);

    // cpu, player のカードのグローバル取得
    let mut cpu_cards = state_cpu_cards.lock().unwrap();
    *cpu_cards = CpuCards::new();
    let mut player_cards: std::sync::MutexGuard<'_, PlayerCards> = state_player_cards.lock().unwrap();
    *player_cards = PlayerCards::new();

    // CPU配布
    for _i in 0..people {
        let mut one_cpu_cards: Vec<(card_color, card_number)> = Vec::new();
        for _j in 0..7 {
            one_cpu_cards.push(deck.draw_card());
        }
        cpu_cards.cards.push(one_cpu_cards);
    }

    // プレイヤー配布
    for i in 0..7 {
        player_cards.cards.push(deck.draw_card());
    }
}

// 一番上のカードを取得する関数 for tauri
#[tauri::command]
fn invoke_get_stage_card(state: State<Mutex<StageCard>>) -> (card_color, card_number){
    let stage_card = state.lock().unwrap();
    stage_card.get_stage_card()
}

#[tauri::command]
fn invoke_get_player_cards(state: State<Mutex<PlayerCards>>) -> Vec<(card_color, card_number)>{
    let player_cards = state.lock().unwrap();
    player_cards.get_cards()
}

#[tauri::command]
fn show_data(state: State<Mutex<GameConfig>>) {
    let data = state.lock().unwrap();
    println!("現在の設定: 人数 = {}, 回数 = {}", data.people, data.play_times);
}

fn main() {
    // colorcards_lib::run()
    tauri::Builder::default()
        .manage(Mutex::new(Deck::new()))
        .manage(Mutex::new(CpuCards::new()))
        .manage(Mutex::new(PlayerCards::new()))
        .manage(Mutex::new(StageCard::new()))
        .manage(Mutex::new(GameConfig { people: 0, play_times: 0 }))
        .invoke_handler(tauri::generate_handler![setup, show_data, invoke_get_stage_card, invoke_get_player_cards])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

