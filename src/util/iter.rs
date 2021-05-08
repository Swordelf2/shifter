/// Returns an iterator over all pairs of consecutive elements in `iter`, including
/// `(last, first)`. So, the total length of returned iterator is equal to the length
/// of `iter`.
pub fn pairs<I>(
    mut iter: impl Iterator<Item = I> + Clone,
) -> impl Iterator<Item = (I, I)> {
    let first = iter.clone();
    let first_last = first.clone().take(1);
    iter.next();
    let second = iter.chain(first_last);
    first.zip(second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_test() {
        assert!(itertools::equal(
            pairs(std::iter::empty::<u32>()),
            std::iter::empty::<(u32, u32)>()
        ));
        let v = vec![0];
        assert!(itertools::equal(
            pairs(v.into_iter()),
            vec![(0, 0)].into_iter()
        ));
        let v = vec![0, 1];
        assert!(itertools::equal(
            pairs(v.into_iter()),
            vec![(0, 1), (1, 0)].into_iter()
        ));
        let v = vec![0, 1, 2, 3, 4, 5];
        assert!(itertools::equal(
            pairs(v.into_iter()),
            vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0)].into_iter()
        ));
    }
}
