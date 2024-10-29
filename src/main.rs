use anyhow::Result;
use concurrency::Matrix;

fn main() -> Result<()> {
    let matrix1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
    let matrix2 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);

    let multiply_res = matrix1 * matrix2;

    println!("{}", multiply_res);
    Ok(())
}
