pub trait Median<T> {
    fn median(&mut self) -> T;
}

impl<T: Copy + Ord> Median<T> for [T] {
    fn median(&mut self) -> T {
        self.sort();

        if self.len() % 2 == 0 {
            let mid = self.len() / 2;

            self[mid - 1]
        } else {
            self[self.len() / 2]
        }
    }
}

pub trait Mean {
    fn mean(&self) -> f64;
}

impl<T> Mean for [T]
where
    T: Copy + Into<f64>,
{
    fn mean(&self) -> f64 {
        self.iter().fold(0f64, |acc, &x| acc + x.into()) / self.len() as f64
    }
}
