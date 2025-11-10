pub struct IncreasingOnly<I, T>
where
    I: Iterator<Item = T>,
    T: PartialOrd + Copy,
{
    iter: I,
    current_max: Option<T>,
}

impl<I, T> IncreasingOnly<I, T>
where
    I: Iterator<Item = T>,
    T: PartialOrd + Copy,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            current_max: None,
        }
    }
}

impl<I, T> Iterator for IncreasingOnly<I, T>
where
    I: Iterator<Item = T>,
    T: PartialOrd + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if self.current_max.map_or(true, |max| item > max) {
                self.current_max = Some(item);
                return Some(item);
            }
        }
        None
    }
}
