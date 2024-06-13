use std::{fmt::{self, Debug, Display}, ops::{Add, AddAssign, Deref, Mul}, usize};
use anyhow::{self, anyhow};

#[derive(Debug)]
struct Matrix<T> where T: Debug + Copy + Mul<Output = T> + Add + AddAssign {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct Vector<T> {
    data: Vec<T>
}

impl<T> Vector<T> {
    fn new(data: impl Into<Vec<T>>) -> Self {
        Vector { data: data.into() }
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> anyhow::Result<T>
where T: Debug + Default + Copy + Mul<Output = T> + Add + AddAssign {
    if a.len() != b.len() {
        return anyhow!("Dot product err: a.len != b.len");
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
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
    //
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    #[test]
    fn test_vector_new() {
        let a = Vector::new(&[1, 2, 3, 5]);
        let b = Vector::new(&[1, 2, 3, 5]);
        assert_eq!(a.len(), b.len());
    }
}
