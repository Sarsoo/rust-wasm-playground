import * as wasm from "wazem";

wasm.test_log();
document.getElementById("clickMe").onclick = OnButtonClick;

export function OnButtonClick(){
    wasm.greet();
}

export default wasm;
