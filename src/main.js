const invoke = window.__TAURI__.core.invoke;

async function setup(){
	let people = document.getElementById("people").value;
	let play_times = document.getElementById("play_times").value;
	if(people <= 1 || people >= 8){
		// CPUは2人以上
		alert("人数が不適切です！");
		return 0;
	}
	if(play_times <= 0 || play_times >= 11){
		alert("プレイ回数が不適切です");
		return 0;
	}
	people = parseInt(people);
	play_times = parseInt(play_times);
	await invoke("setup", {people: people, playTimes: play_times});
	location.href="play_view.html";
}

