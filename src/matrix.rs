use std::{
    fmt::{self},
    ops::{Add, AddAssign, Mul},
    sync::mpsc::{self},
    thread,
};

use crate::{vector::dot_product, Vector};
use anyhow::anyhow;

const THREAD_NUM: usize = 4;

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}
impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix[{}*{}]{{", self.row, self.col)?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }
            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> Mul for Matrix<T>
where
    T: Add<Output = T> + AddAssign + Default + Mul<Output = T> + Copy + Send + 'static,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix multiplication failed")
    }
}
struct MsgInput<T> {
    row: Vector<T>,
    col: Vector<T>,
    idx: usize,
}

impl<T> MsgInput<T> {
    fn new(row: Vector<T>, col: Vector<T>, idx: usize) -> Self {
        Self { row, col, idx }
    }
}

// communication data structure wrapped Vectors to do dot product
struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutPut<T>>,
}

impl<T> Msg<T> {
    fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutPut<T>>) -> Self {
        Msg { input, sender }
    }
}

struct MsgOutPut<T> {
    dot_prd: T,
    idx: usize,
}

// a function to multiply two Matrixs
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: Add<T, Output = T> + Copy + AddAssign + Default + Mul<T, Output = T> + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error: dimensions do not match"));
    }
    let senders = (0..THREAD_NUM)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(|| {
                for msg in rx {
                    let dot_prd = dot_product(&msg.input.row, &msg.input.col);
                    if let Err(e) = msg.sender.send(MsgOutPut {
                        dot_prd,
                        idx: msg.input.idx,
                    }) {
                        eprintln!("send error: {}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let mut receivers = Vec::new();
    for i in 0..a.row {
        for j in 0..b.col {
            let row_data = &a.data[i * a.col..(i + 1) * a.col];
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let row = Vector::new(row_data);
            let col = Vector::new(col_data);
            // send Msg to thread
            let idx = i * a.row + j;
            let (tx, rx) = oneshot::channel();
            let msg_input = MsgInput::new(row, col, idx);
            let msg = Msg::new(msg_input, tx);
            if let Err(e) = senders[idx % THREAD_NUM].send(msg) {
                eprintln!("send error: {}", e);
            }
            receivers.push(rx);
        }
    }
    let mut data = vec![T::default(); a.row * b.col];
    for receiver in receivers {
        let msg_output = receiver.recv()?;
        data[msg_output.idx] = msg_output.dot_prd;
    }
    Ok(Matrix::new(data, a.row, b.col))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![7, 8, 9, 10, 11, 12], 3, 2);
        let c = a * b;
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 2);
        assert_eq!(c.data, vec![58, 64, 139, 154]);
        println!("{c}");
    }

    #[test]
    fn test_a_can_not_multiply_b() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let c = multiply(&a, &b);
        assert!(c.is_err());
    }

    #[test]
    #[should_panic]
    fn test_a_can_not_multiply_b_panic() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let _c = a * b;
    }
}
