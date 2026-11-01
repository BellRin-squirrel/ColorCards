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
    if(enum_to_display[stage_card[1]] == undefined){
        el_stage_card.textContent = stage_card[1];
    } else{
        el_stage_card.textContent = enum_to_display[stage_card[1]];
    }
    el_stage_card.style.background = color_codes(stage_card[0]);

    // プレイヤーのカードを表示
    let player_cards = await invoke("invoke_get_player_cards");
    for(let card of player_cards) {
        // 描画
        let player_card = document.createElement("div");
        player_card.classList.add("card");
        player_card.classList.add("player_card");
        if(enum_to_display[card[1]] == undefined){
            player_card.textContent = card[1];
        } else{
            player_card.textContent = enum_to_display[card[1]];
        }
        player_card.style.background = color_codes(card[0]);
        console.log(color_codes(card[0]));
        el_player_cards.appendChild(player_card);
    }
});

function color_codes(color) {
    switch(color) {
        case "Blue": return "#0000ff";
        case "Green": return "#00ff00";
        case "Red": return "#ff0000";
        case "Yellow": return "#f6d727";
        default: return "linear-gradient(to bottom right,red,orange,yellow,green,blue,indigo,violet)";
    }
}
