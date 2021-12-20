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
// 定义一个包含单个 `Vec` 的结构体 `List`。
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个 `vec` 的引用。
        let vec = &self.0;

        write!(f, "[")?;

        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{} : {}", count, v)?;
        }

        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
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

    let v = List(vec![1, 2, 3]);

    println!("Display {}", complex);
    println!("Debug {:?}", complex);
    println!("List is {}", v);
}
