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
});
