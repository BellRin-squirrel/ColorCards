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
    "WildShuffle": "H"
}

document.addEventListener("DOMContentLoaded", async function () {
    const el_stage_card = document.getElementById("top_card");
    const el_player_cards = document.getElementById("player_cards");

    console.log("[Debug] 描画開始");
    
    // ステージのカードを表示
    let stage_card = await invoke("invoke_get_stage_card"); // 0 が色, 1 がカードの種類
    console.log("[Debug] Invoke 完了");
    el_stage_card.textContent = enum_to_display[stage_card[1]] == undefined ? stage_card[1] : enum_to_display[stage_card[1]];
    el_stage_card.classList.add(stage_card[0]);

    // プレイヤーのカードを表示
    let player_cards = await invoke("invoke_get_player_cards");
    for(let card of player_cards) {
        // 描画
        let player_card = document.createElement("div");
        player_card.classList.add("card");
        player_card.classList.add("player_card");
        player_card.classList.add(card[0]);
        player_card.textContent = enum_to_display[card[1]] == undefined ? card[1] : enum_to_display[card[1]];
        el_player_cards.appendChild(player_card);
    }

    // ゲームプレイ
    gameLoop();
});

// ゲームプレイループ
async function gameLoop() {
    let gameOver = false;

    while (!gameOver) {
        // 1. 現在のターン情報を取得
        let turnInfo = await invoke("invoke_get_current_turn"); 

        console.log(turnInfo.is_player);

        if (turnInfo.is_player) {
            // --- 人間のターン ---
            console.log("あなたの番です。カードをクリックしてください。");
            // カードがクリックされるまで待機（Promise を使う）
            let selectedCardIndex = await waitForPlayerAction();
            
            // Rustに出す命令を送る（出せるかどうかもRustでチェック）
            let result = await invoke("invoke_player_play_card", { cardIndex: selectedCardIndex });
            if (!result.success) {
                alert("そのカードは出せません！");
                continue; // やり直し
            }
        } else {
            // --- CPUのターン ---
            console.log(`CPU ${turnInfo.cpu_id} の番です...`);
            // CPUが考えているように見せるため1秒待つ演出
            await new Promise(r => setTimeout(r, 1000));

            // Rust側でCPUの1手（思考とカード出し）を実行してもらう
            await invoke("invoke_cpu_play_turn", { cpuId: turnInfo.cpu_id });
        }

        // 2. 画面を再描画（場札、手札、CPUの残り枚数など）
        await updateUI();

        // 3. 勝敗判定
        gameOver = await invoke("invoke_check_game_over");
    }

    alert("ゲーム終了！");
}

function waitForPlayerAction() {
    return new Promise((resolve) => {
        // 画面上にある手札のカード要素をすべて取得
        const cards = document.querySelectorAll(".player_card");

        cards.forEach((cardElement, index) => {
            // 一度だけ発火するように { once: true } を付けると綺麗に処理できます
            cardElement.addEventListener("click", () => {
                // クリックされたカードの番号 (0, 1, 2...) を渡して Promise を完了させる
                resolve(index);
            }, { once: true });
        });
    });
}

function updateUI() {

}
