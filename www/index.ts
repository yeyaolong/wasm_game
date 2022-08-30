import init, { World, Direction } from "wasm_game";
import { random } from "./util/random";

init().then((wasm) => {
    const CELL_SIZE = 20; // 定义一个格子的大小
    const WORLD_WIDTH = 16;
    const SNAKE_HEAD_INDEX = random(WORLD_WIDTH * WORLD_WIDTH);
    const world = World.new(WORLD_WIDTH, SNAKE_HEAD_INDEX); // 一个16 * 16 的世界, 蛇的位置在SNAKE_HEAD_INDEX
    const worldWidth = world.width();
    const fps = 5;
    const canvas = <HTMLCanvasElement>document.getElementById("snake-world");

    const context = canvas.getContext("2d");

    canvas.width = worldWidth * CELL_SIZE;
    canvas.height = worldWidth * CELL_SIZE;


    document.addEventListener("keydown", e => {
        switch(e.code) {
            case "ArrowUp":
                world.change_snake_direction(Direction.Up);
                break;
            case "ArrowDown":
                world.change_snake_direction(Direction.Down);
                break;
            
            case "ArrowLeft":
                world.change_snake_direction(Direction.Left);
                break;
            case "ArrowRight":
                world.change_snake_direction(Direction.Right);
                break;
        }
    })

    // 画一个16 * 16的网格
    function drawWorld() {
        context.beginPath();
         // 画横线
        for (let x = 0; x <= worldWidth; x++) {
            context.moveTo(CELL_SIZE * x, 0); 
            context.lineTo(CELL_SIZE * x, CELL_SIZE * worldWidth);
        }
         // 画竖线
        for (let y = 0; y <= worldWidth; y++) {
            context.moveTo(0, CELL_SIZE * y);
            context.lineTo(CELL_SIZE * worldWidth, CELL_SIZE * y);
        }

        context.stroke();
        
    }

    // 画蛇头
    function drawSnake() {

        const snakeCells = new Uint32Array(
            wasm.memory.buffer,
            world.snake_cells(), // 指针
            world.snake_length(), // 长度
        ); // 用unit32Array接收原生指针

        snakeCells.forEach((cell, i) => {
            const col = cell % worldWidth;
            const row = Math.floor(cell / worldWidth);
            context.beginPath();
            // 蛇头与蛇身区分颜色
            i == 0 ? context.fillStyle = '#787878' : context.fillStyle = '#000000';
            
            context.fillRect(
                col * CELL_SIZE, // x
                row * CELL_SIZE, // y
                CELL_SIZE,
                CELL_SIZE
            );
        });
        context.stroke();
    }
    // 画一个蛋
    function drawReward() {
        const index = world.reward_cell();
        const row = Math.floor(index / worldWidth);
        const col = index % worldWidth;
        context.beginPath();
        context.fillStyle = "#ff0000"; // 给蛋的颜色设置成红色
        context.fillRect(
            col * CELL_SIZE, // x
            row * CELL_SIZE, // y
            CELL_SIZE,
            CELL_SIZE
        )


        context.stroke();

        if (index === 123456789) {
            alert("Won")
        }
    }


    function draw() {
        drawWorld();
        drawSnake();
        
        drawReward();
    }

    function run() {
        // 视频里用的setTimeout里套requestAnimationFrame,
        // 虽然我觉得直接window.requestAnimationFrame会更好一点
        // 但实际上，requestAnimationFrame似乎有点太快了
        // 所以这里只好外层套一个setTimeout用来减速
        setTimeout(() => {
            context.clearRect(0, 0, canvas.width, canvas.height);
            world.update();
            draw();
            requestAnimationFrame(run)
        }, 1000 / fps);
    }

    draw();
    run();

})