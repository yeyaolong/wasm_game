import init, { World } from "wasm_game";

init().then(() => {
    const CELL_SIZE = 20; // 定义一个格子的大小
    
    const world = World.new(8); // 一个16 * 16 的世界
    const worldWidth = world.width();
    const fps = 5;
    const canvas = document.getElementById("snake-world");

    const context = canvas.getContext("2d");

    canvas.width = worldWidth * CELL_SIZE;
    canvas.height = worldWidth * CELL_SIZE;
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
        const snake_index = world.snake_head_index();
        const row = Math.floor(snake_index / worldWidth);
        const col = snake_index % worldWidth;

        context.beginPath();

        context.fillRect(
            col * CELL_SIZE, // x
            row * CELL_SIZE, // y
            CELL_SIZE,
            CELL_SIZE
        )


        context.stroke();
    }


    function draw() {
        drawWorld();
        drawSnake();
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