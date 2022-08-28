use wasm_bindgen::prelude::*; // JS与Rust交互的包
extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(name);
}


#[wasm_bindgen]
pub struct World {
    // 定义一个结构体，这个结构体包含两个参数，width和size
    // 两个参数的类型都是usize类型(无符号整型数据，位数大小由操作系统决定，比如我的电脑是64位的，那usize就占有64bit)
    width: usize,
    size: usize,
    snake: Snake,
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,

}

impl Snake {
    /**
      * spawn_index 初始点位
      */
    fn new(spawn_index: usize) -> Self {
        Self {
            body: vec![SnakeCell(spawn_index)],
        }
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            size: width * width, // 创建一个长宽相等的正方形
            snake: Snake::new(13), // 创建一条在13位置的蛇
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        // 不要写分号，不然报错
        self.snake.body[0].0
    }
    pub fn update(&mut self) {
        let snake_head_index: usize = self.snake_head_index();
        self.snake.body[0].0 = (snake_head_index+1) % self.size;
    }
}
