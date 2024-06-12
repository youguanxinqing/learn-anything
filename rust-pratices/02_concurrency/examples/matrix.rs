use std::{fmt::{self, Debug, Display}, ops::{Add, AddAssign, Mul}, usize};

#[derive(Debug)]
struct Matrix<T> where T: Debug + Copy + Mul<Output = T> + Add + AddAssign {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T> Matrix<T> where T: Debug + Copy + Mul<Output = T> + Add + AddAssign {
    fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Matrix{
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> fmt::Display for Matrix<T> where T: Debug + Copy + Mul<Output = T> + Add + AddAssign + Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut chunks = Vec::new();
        for i in 0..self.row {
            chunks.push("\n".to_string());
            for j in 0..self.col {
                if j == 0 {
                    chunks.push("[".to_string());
                }
                chunks.push(format!("{}", self.data[i * j]));
            }
            chunks.push("]".to_string());
        }

        write!(f, "{}", chunks.join(" "))
    }
}

fn multify<T>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T>
where T : Debug + Copy + Mul<Output = T> + Add + AddAssign {
    let mut c = Vec::with_capacity(a.row * b.col);
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                c[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }
    
    Matrix::new(c, a.row, b.col)
}

fn main() {
    let a = Matrix::new(&[1, 2, 3, 4], 2, 2);
    // let b = Matrix::new(&[1, 2], 1, 2);
    // let c = multify(&a, &b);

    println!("{}", a);
}
