use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (i, b) = pair;

    (b, i)
}

fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}

fn main() {
    let p = (123, true);

    println!("{:?}", p);

    let rp = reverse(p);

    println!("{:?}", rp);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("{:?}", matrix);
    println!("{}", matrix);

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
