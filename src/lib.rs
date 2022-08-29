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
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
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
    direction: Direction

}

impl Snake {
    /**
      * spawn_index 初始点位
      */
    fn new(spawn_index: usize) -> Self {
        Self {
            body: vec![SnakeCell(spawn_index)],
            // direction: Direction::Down,
        }
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> Self {
        Self {
            width,
            size: width * width, // 创建一个长宽相等的正方形
            snake: Snake::new(snake_index), // 创建一条在13位置的蛇
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        // 不要写分号，不然报错
        self.snake.body[0].0
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }
    /**
      * 将数字映射到二维网格
      * 比如一个 8 * 8 的网格,行和列的下标都从0开始
      * 那么数字13，就可以映射到这个网格的第1行，第4列 (1, 4)
      * (行, 列)
      */
    pub fn index_to_cell(&self, index:usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }
    /**
      * 将二维网格映射到一个数字
      */
    pub fn cell_to_index(&self, row:usize, col: usize) -> usize {
        (row * self.width) + col
    }

    pub fn set_snake_head(&mut self, index: usize) {
        self.snake.body[0].0 = index;
    }

    pub fn update(&mut self) {
        let snake_head_index: usize = self.snake_head_index();
        let (row, col) = self.index_to_cell(snake_head_index); // 拿到蛇头的行和列

        let (row, col) = match self.snake.direction {
            Direction::Left => (row, (col - 1)%self.width),
            Direction::Right => (row, (col + 1)%self.width),
            Direction::Top => ((row -1) % self.width, col), // 这里其实应该用self.height更容易理解，不过因为是width*width的网格，所以width和height值相等
            Direction::Down => ((row + 1) % self.width, col),
        };

        let next_index = self.cell_to_index(row, col);
        self.set_snake_head(next_index);
    }
}
