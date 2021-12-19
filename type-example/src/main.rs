use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
  let num = 8i32;  
  println!("number is {}", num);
  println!("binary And is {:b}", 0b1010 & 0b1101);
  println!("1 << 2 is {}", 1 << 2);
  println!("1 << 2 is {:b}", 1 << 2);
  println!("10 >> 2 is {:b}", 10 >> 2);

  // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
  // println!("too long tuple: {:?}", too_long_tuple);

  // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
  println!("one element tuple: {:?}", (5u32,));
  println!("just an integer: {:?}", (5u32));
  println!("binary format {}", 5f32);

  impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "( {}, {} ) \n", self.0, self.1)?;
      write!(f, "( {}, {} ) \n", self.2, self.3)
    }
  }

  fn transpose(matrix: Matrix) -> Matrix {
    let Matrix(left_top, right_top, left_bottom, right_bottom) = matrix;

    Matrix(left_top,left_bottom ,right_top , right_bottom)
  }

  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

  println!("Matrix: \n{}", matrix);
  println!("Transpose: \n{}", transpose(matrix));
}
