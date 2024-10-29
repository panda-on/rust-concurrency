use std::ops::{Add, AddAssign, Deref, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}
impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}

impl<T> Deref for Vector<T>
where
    T: Add<T, Output = T> + Copy + AddAssign + Default + Mul<T, Output = T>,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        self.data.as_ref()
    }
}

pub fn dot_product<T>(a: &Vector<T>, b: &Vector<T>) -> T
where
    T: Add<T, Output = T> + Copy + AddAssign + Default + Mul<T, Output = T>,
{
    let mut ret = T::default();
    for i in 0..a.len() {
        ret += a[i] * b[i];
    }
    ret
}
