import init, { hello } from "wasm_game";

init().then(() => {
    hello("KappaFish");
    console.log("Ok");
})