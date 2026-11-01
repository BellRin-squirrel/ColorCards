use std::sync::Mutex;
use tauri::State;
use rand::seq::SliceRandom; // シャッフルに使う機能
use rand::rng;
use serde::Serialize;

struct GameConfig {
    people: u32,
    play_times: u32
}

#[tauri::command]
fn setup(people: u32, play_times: u32, state_game_config: State<Mutex<GameConfig>>, state_deck: State<Mutex<Deck>>){
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
    // シャッフル完了
    let top_card = deck.get_top_card();
    println!("{:?}", top_card.0);
    println!("{:?}", top_card.1);
}

#[tauri::command]
fn invoke_get_top_card(state: State<Mutex<Deck>>) -> (card_color, card_number){
    let deck = state.lock().unwrap();
    deck.get_top_card()
}

#[tauri::command]
fn show_data(state: State<Mutex<GameConfig>>) {
    let data = state.lock().unwrap();
    println!("現在の設定: 人数 = {}, 回数 = {}", data.people, data.play_times);
}

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

    fn get_top_card(&self) -> (card_color, card_number) {
        self.cards[0]
    }
}

fn main() {
    // colorcards_lib::run()
    tauri::Builder::default()
        .manage(Mutex::new(GameConfig { people: 0, play_times: 0 }))
        .manage(Mutex::new(Deck::new()))
        .invoke_handler(tauri::generate_handler![setup, show_data, invoke_get_top_card])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

