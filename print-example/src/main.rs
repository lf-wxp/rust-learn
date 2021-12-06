use std::fmt; // 导入 `fmt`

// 为了比较，定义一个含有具名字段的结构体。
#[derive(Debug)]
struct Complex {
    x: f64,
    y: f64,
}

// 类似地对 `Complex` 实现 `Display`
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义格式，使得仅显示 `x` 和 `y` 的值。
        write!(f, "{} + {}i", self.x, self.y)
    }
}

fn main() {
    println!("Hello, world!");
    println!("{} of {:b} people know binary, the other half don't", 1, 8);
    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    println!("This struct `{:?}` won't print...", Structure(3));

    let pi = 3.1415926;
    println!("pi is {:.*}", 3, pi);

    let complex = Complex {
      x: 3.3,
      y: 7.2,
    };

    println!("Display {}", complex);
    println!("Debug {:?}", complex);
}
