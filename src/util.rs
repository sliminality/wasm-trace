#[derive(Debug)]
/// Allows us to wrap and return `impl Iterator` trait objects
/// with different types, namely `iter::empty` and other iterators.
pub enum Either<A, B> {
    A(A),
    B(B),
}

impl<A: Iterator, B: Iterator<Item = A::Item>> Iterator for Either<A, B> {
    type Item = B::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::A(it) => it.next(),
            Either::B(it) => it.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Either::A(_) => (0, Some(0)),
            Either::B(it) => it.size_hint(),
        }
    }
}
