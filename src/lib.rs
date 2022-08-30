use wasm_bindgen::prelude::*; // JS与Rust交互的包
extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}
#[wasm_bindgen(module = "/www/util/random.ts")]
extern {
    fn random(max: usize) -> usize; // 导入js的函数
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



#[derive(PartialEq, Clone, Copy)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction

}

/**
  * 蛇头在下一帧的位置，由Direction和当前位置决定。
  * 蛇身在下一帧的位置，等于这一帧上一节身体的位置
  */

impl Snake {
    /**
      * spawn_index 蛇头初始点位
      * size 蛇身大小(包含蛇头)
      */
    fn new(spawn_index: usize, size: usize) -> Self {
        let mut body = Vec::new();
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }
        Self {
            body: body,
            direction: Direction::Down,
        }
    }
}


#[wasm_bindgen]
pub struct World {
    // 定义一个结构体，这个结构体包含两个参数，width和size
    // 两个参数的类型都是usize类型(无符号整型数据，位数大小由操作系统决定，比如我的电脑是64位的，那usize就占有64bit)
    width: usize,
    size: usize,
    reward_cell: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>
    
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> Self {
        let size = width * width;
        let snake = Snake::new(snake_index, 3);
        Self {
            width,
            size: size, // 创建一个长宽相等的正方形
            // snake: snake, // 创建一条在13位置的蛇, 不能定义在这里
            reward_cell: World::gen_reward_cell(size, &snake.body),
            snake: snake, // 创建一条在13位置的蛇, // 必须写在reward_cell下面
            next_cell: None
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        // 不要写分号，写了分号就不是return了
        // 不写分号的语句会被return
        self.snake.body[0].0
    }
    /**
      * 改变蛇的运动方向
      */
    pub fn change_snake_direction(&mut self, direction: Direction) {
        // 正在向左 不能向右
        // 并且正在向下，不能向上
        let next_cell = self.gen_next_snake_cell(&direction);
        if self.snake.body[1].0 == next_cell.0 {
            return;
        }
        
        self.snake.direction = direction;
    }
    /**
      * 返回蛇身长度
      */
    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    /**
      * 将数字映射到二维网格
      * 比如一个 8 * 8 的网格,行和列的下标都从0开始
      * 那么数字13，就可以映射到这个网格的第1行，第4列 (1, 4)
      * (行, 列)
      */
      // 不理解，这里为啥不能写成 pub fn index_to_cell
    fn index_to_cell(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }
    /**
      * 将二维网格映射到一个数字
      */
    fn cell_to_index(&self, row:usize, col: usize) -> usize {
        (row * self.width) + col
    }

    pub fn set_snake_head(&mut self, index: usize) {
        self.snake.body[0].0 = index;
    }
    // 拿到蛇身（的指针）
    // 数组的指针也就是数组的头一个元素的指针
    // 也就是蛇头位置的指针
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn update(&mut self) {

        let temp: Vec<SnakeCell> = self.snake.body.clone();
        // 使用Option提高性能
        match self.next_cell {
            // 如果有值
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            },
            // 如果没有值
            None => {
                self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
            }
        }
        let len = self.snake.body.len();
        // 从1开始，因为0是蛇头
        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i-1].0);
        }
        // 蛇头碰到了，说明吃到了
        if self.reward_cell == self.snake_head_index() {
            if self.snake_length() < self.size {
                // 重新生成一个蛋
                self.reward_cell = World::gen_reward_cell(self.size, &self.snake.body);
                // 蛇身体变长
                self.snake.body.push(SnakeCell(self.snake.body[1].0)); // 移动第二个元素
            } else {
                // 如果蛇身体与世界一样大了,游戏胜利，胜利的判断条件是 rewrd_cell = 123456789
                self.reward_cell = 123456789;
            }
            
        }
    }
    /**
      * 蛇下一帧的位置
      */
    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_index = self.snake_head_index();
        let row = snake_index / self.width;

        return match direction {
            Direction::Up => {
                // 上边界
                let border_hold: usize = snake_index - ((self.width - row) * self.width);
                if snake_index == border_hold {
                    // 等于上边界，则从下边界穿透出来
                    SnakeCell((self.size - self.width) + border_hold)
                } else {
                    SnakeCell(snake_index - self.width)
                }
            }
            Direction::Down => {
                // 下边界
                let border_hold: usize = snake_index + ((self.width - row) * self.width);
                if snake_index == border_hold {
                    // 等于下边界，则从上边界穿透出来
                    SnakeCell(border_hold - (row * self.width))
                } else {
                    SnakeCell(snake_index + self.width)
                }
            }
            Direction::Left => {
                let border_hold: usize = row * self.width;
                if snake_index == border_hold {
                    SnakeCell(border_hold + self.width - 1)
                } else {
                    SnakeCell(snake_index - 1)
                }
            }
            Direction::Right => {
                let border_hold: usize =  (row + 1) * self.width;
                if (snake_index + 1) == border_hold {
                    SnakeCell(border_hold - self.width)
                } else {
                    SnakeCell(snake_index + 1)
                }
            }
        }
    }
    // 在随机位置生成一颗蛋
    // 蛋和蛇身体不能重合
    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell;
        loop {
            reward_cell = random(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }
        reward_cell
    }
    // fn gen_reward_cell(max: usize) -> usize {
    //     random(max)
    // }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }
}
