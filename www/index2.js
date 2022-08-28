/**
 * @description js调用wasm
 */
async function run() {

    const response = await fetch('./test.wasm');

    const buffer = await response.arrayBuffer();

    const wasm = await WebAssembly.instantiate(buffer);

    const addTwoFunction = wasm.instance.exports.addTwo;

    const result = addTwoFunction(10, 20);

    console.log(result);
}

run();

/**
 * @description wasm调用js
 */
 async function run2() {
    // wasm的写法有点像汇编，写起来不舒服。
    /*
        (module
        (import "console" "log" (func $log))
        (import "console" "error" (func $error))
        (func (export "addThree") (param i32 i32) (result i32)
            local.get 0
            local.get 1
            call $log
            call $error
            i32.add))
    
    */
    const importObject = {
        console: {
            log: ()=> {
                console.log("log info!");
            },
            error: () => {
                console.error("error info");
            }
        }
    }

    const response = await fetch('./test2.wasm');

    const buffer = await response.arrayBuffer();

    const wasm = await WebAssembly.instantiate(buffer, importObject);

    const addThreeFunction = wasm.instance.exports.addThree;

    const result = addThreeFunction(10, 20);

    console.log(result);
}

run();

run2();
