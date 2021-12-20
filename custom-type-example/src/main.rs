// 带有两个字段（field）的结构体
#[derive(Debug)]
struct Point {
  x: f32,
  y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
  p1: Point,
  p2: Point,
}
impl Rectangle {
  fn rect_area(&self) -> f32 {
    let Rectangle { p1: Point {x: x1, y: y1 }, p2: Point { x: x2, y: y2 }} = self;
    (x2 - x1) * (y2 - y1)
  }

  fn square(point: Point, size: f32) -> Rectangle {
    let Point { x, y } = point;
    let point1 = Point {
      x,
      y: size - y,
    };
    let point2 = Point {
      x: x + size,
      y,
    };

    Rectangle {
      p1: point1,
      p2: point2,
    }
  }
}

fn main() {
  let rect = Rectangle {
    p1: Point {x: 0.0, y: 0.0},
    p2: Point { x: 10.0, y: 20.0 },
  };

  println!("rect_area is {}", rect.rect_area());

  println!("new rectangle is {:#?}", Rectangle::square(Point { x: 5.0, y: 5.0 }, 10.0));
}
