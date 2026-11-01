const invoke = window.__TAURI__.core.invoke;
const color_codes = {
    "Blue": "#0000ff",
    "Green": "#00ff00",
    "Red": "#ff0000",
    "Yellow": "#f6d727"
}
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
    "Zero": 0
}

document.addEventListener("DOMContentLoaded", async function () {
    console.log("[Debug] 描画開始");
    const top_card = document.getElementById("top_card");
    
    // ステージのカードを表示
    let response = await invoke("invoke_get_stage_card"); // 0 が色, 1 がカードの種類
    console.log("[Debug] Invoke 完了");
    if(enum_to_display[response[1]] == undefined){
        top_card.textContent = response[1];
    } else{
        top_card.textContent = enum_to_display[response[1]];
    }
    top_card.style.backgroundColor = color_codes[response[0]];

    // プレイヤーのカードを表示
});
