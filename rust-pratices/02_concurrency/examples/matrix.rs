use anyhow::anyhow;
use std::{
    fmt::{self, Debug, Display}, ops::{Add, AddAssign, Deref, Mul}, sync::mpsc, thread, usize
};

const MAX_PROCESS: usize = 4;

#[derive(Debug)]
struct Matrix<T>
where
    T: Debug + Copy + Mul<Output = T> + Add + AddAssign,
{
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct Vector<T> {
    data: Vec<T>,
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

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

impl<T> MsgInput<T> {
    fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        MsgInput { idx, row, col }
    }
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Msg { input, sender }
    }
}

fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> anyhow::Result<T>
where
    T: Debug + Default + Copy + Mul<Output = T> + Add + AddAssign,
{
    if a.len() != b.len() {
        anyhow::bail!("{}", "Dot product err: a.len != b.len");
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}

impl<T> Matrix<T>
where
    T: Debug + Copy + Mul<Output = T> + Add + AddAssign,
{
    fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Matrix {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: Debug + Copy + Mul<Output = T> + Add + AddAssign + Display,
{
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

fn multify<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: Debug + Copy + Mul<Output = T> + Add + AddAssign + Default + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error: a.col != b.row"));
    }

    let senders: Vec<mpsc::Sender<Msg<_>>> = (0..MAX_PROCESS).map(|_| -> mpsc::Sender<_> {
        let (tx, rx) = mpsc::channel::<Msg<_>>();
        thread::spawn(move || {
            for msg in rx {
                let value = dot_product(msg.input.row, msg.input.col).unwrap();
                if let Err(e) = msg.sender.send(MsgOutput { idx: msg.input.idx, value }) {
                    eprintln!("send output err: {}", e);
                }
            }
        });
        tx
    }).collect();

    let mut receivers = vec![];
    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            // c[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()

                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);

            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % MAX_PROCESS].send(msg) {
                eprintln!("send input err: {}", e);
            } else {
                receivers.push(rx);
            }
        }
    }

    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
    }

    Ok(Matrix::new(data, a.row, b.col))
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
