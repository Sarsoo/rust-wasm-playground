import * as wasm from "wazem";

wasm.test_log();
document.getElementById("clickMe").onclick = OnButtonClick;
document.getElementById("minButton").onclick = OnMinClick;

export function OnButtonClick(){
    wasm.greet();
}

export function OnMinClick(){
    wasm.find_min();
}

export default wasm;
