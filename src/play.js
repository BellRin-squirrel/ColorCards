const invoke = window.__TAURI__.core.invoke;
const enum_to_display = {
    "One": 1,
    "Two": 2,
    "Three": 3,
    "Four": 4,
    "Five": 5,
    "Six": 6,
    "Seven": 7,
    "Eight": 8,
    "Nine": 9,
    "Zero": 0,
    "Skip": "S",
    "Reverse": "R",
    "Draw": "D",
    "Wild": "w",
    "WildDraw": "W",
    "ShuffleWild": "H"
};

let el_stage_card = null;
let el_player_cards = null;

document.addEventListener("DOMContentLoaded", async function () {
    el_stage_card = document.getElementById("top_card");
    el_player_cards = document.getElementById("player_cards");
    console.log("[Debug] 描画開始");
    
    // ステージのカードを表示
    let stage_card = await invoke("invoke_get_stage_card");
    console.log("[Debug] Invoke 完了");
    el_stage_card.textContent = enum_to_display[stage_card[1]] === undefined ? stage_card[1] : enum_to_display[stage_card[1]];
    el_stage_card.classList.add(stage_card[0]);

    // プレイヤーのカードを表示
    let player_cards = await invoke("invoke_get_player_cards");
    display_player_cards(player_cards);

    // ゲームプレイ開始
    gameLoop();
});

// ゲームプレイループ
async function gameLoop() {
    let gameOver = false;

    while (!gameOver) {
        // 1. 現在のターン情報を取得
        let turnInfo = await invoke("invoke_get_current_turn"); 

        if (turnInfo.is_player) {
            // --- 人間のターン ---
            console.log("あなたの番です。カードを出すか、スキップを押してください。");
            
            // ★ カードまたはスキップボタンが押されるまで待機
            let action = await waitForPlayerAction();
            
            if (action.type === "play") {
                // カードを出そうとした場合
                let result = await invoke("invoke_player_play_card", { cardIndex: action.index });
                if (!result.success) {
                    alert("そのカードは出せません！");
                    continue; // 出せなかったらもう一度プレイヤーの番をやり直し
                }
            } else if (action.type === "skip") {
                // スキップボタンが押された場合
                console.log("スキップを選択しました");
                await invoke("invoke_player_skip");
            }
        } else {
            // --- CPUのターン ---
            console.log(`CPU ${turnInfo.cpu_id} の番です...`);
            // CPUが考えているように見せるため1秒待つ演出
            await new Promise(r => setTimeout(r, 1000));

            // Rust側でCPUの1手（思考とカード出し）を実行
            await invoke("invoke_cpu_play_turn", { cpuId: turnInfo.cpu_id });
        }

        // 2. 画面を再描画（場札、手札、CPUの残り枚数など）
        await updateUI();

        // 3. 勝敗判定
        gameOver = await invoke("invoke_check_game_over");
    }

    alert("ゲーム終了！");
}

// ★ プレイヤーの操作（カードクリック または スキップクリック）を待つ関数
function waitForPlayerAction() {
    return new Promise((resolve) => {
        // 二重クリック防止用のコントローラー
        const controller = new AbortController();
        const signal = controller.signal;

        // 1. 各手札カードのクリック待ち
        const cards = document.querySelectorAll(".player_card");
        cards.forEach((cardElement, index) => {
            cardElement.addEventListener("click", () => {
                controller.abort(); // 1回押されたらすべてのクリック待ちを解除
                resolve({ type: "play", index: index });
            }, { signal });
        });

        // 2. スキップボタンのクリック待ち
        const skipBtn = document.getElementById("skip");
        if (skipBtn) {
            skipBtn.addEventListener("click", () => {
                controller.abort(); // 1回押されたらすべてのクリック待ちを解除
                resolve({ type: "skip" });
            }, { signal });
        }
    });
}

function display_player_cards(cards) {
    // 既存の描画コンテンツを消去
    el_player_cards.innerHTML = "";
    // 再描画
    for (let card of cards) {
        let player_card = document.createElement("div");
        player_card.classList.add("card");
        player_card.classList.add("player_card");
        player_card.classList.add(card[0]);
        player_card.textContent = enum_to_display[card[1]] === undefined ? card[1] : enum_to_display[card[1]];
        el_player_cards.appendChild(player_card);
    }
}

async function updateUI() {
    let info = await invoke("invoke_update_ui");
    let stage_card_number = info.stage_card[1];
    let stage_card_color = info.current_color;
    let player_cards = info.player_cards;

    // 場のカードの文字を変更
    el_stage_card.textContent = enum_to_display[stage_card_number] === undefined ? stage_card_number : enum_to_display[stage_card_number];

    // 場のカードの色を変更
    el_stage_card.className = `card ${stage_card_color}`;

    // プレイヤーの手持ちカードを再描画
    display_player_cards(player_cards);
}