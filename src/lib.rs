#![no_std]
#![doc = include_str!("../README.md")]
extern crate alloc;

#[doc(hidden)]
enum ChainFromLastState<A: Iterator + Sized, B, F> {
    A {
        a: A,
        f: Option<F>,
        prev: Option<A::Item>,
    },
    B(B),
}

/// An iterator adaptor that chains the iterator with an iterator built from the last item.
///
/// See [`.chain_from_last(...)`](ChainFromLastExt::chain_from_last) for more information.
#[must_use = "Iterators adaptors are lazy and do nothing unless consumed"]
#[repr(transparent)]
pub struct ChainFromLast<A: Iterator + Sized, B, F>(ChainFromLastState<A, B, F>);

pub trait ChainFromLastExt: Iterator {
    /// Returns an iterator adaptor that chains an iterator with iterator built
    /// from the last item.
    ///
    /// If the iterator is empty this is no-op.
    ///
    /// ```
    /// # use chain_from_last::ChainFromLastExt;
    /// let words: Vec<_> = "lorem ipsum dolor;sit;amet"
    ///     .split(" ")
    ///     .chain_from_last(|l| l.split(";"))
    ///     .collect();
    ///
    ///  assert_eq!(words, vec!["lorem", "ipsum", "dolor", "sit", "amet"]);
    /// ```
    fn chain_from_last<B: Iterator<Item = Self::Item>, F: FnOnce(Self::Item) -> B>(
        self,
        f: F,
    ) -> ChainFromLast<Self, B, F>
    where
        Self: Sized,
    {
        ChainFromLast(ChainFromLastState::A {
            a: self,
            f: Some(f),
            prev: None,
        })
    }
}

impl<I: Iterator> ChainFromLastExt for I {}

impl<A: Iterator + Sized, B: Iterator<Item = A::Item>, F: FnOnce(A::Item) -> B> Iterator
    for ChainFromLast<A, B, F>
{
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            ChainFromLastState::A { a, f, prev } => {
                // get the previous item or yield the first item if exists.
                let prev_item = prev.take().or_else(|| a.next())?;

                let next = a.next();

                // if no more items call the callback with the last item.
                if next.is_none() {
                    *self = Self(ChainFromLastState::B(f.take().unwrap()(prev_item)));
                    return self.next();
                }

                *prev = next;
                Some(prev_item)
            }
            ChainFromLastState::B(b) => b.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ChainFromLastExt;

    #[test]
    fn one_item() {
        let mut i = core::iter::once('a').chain_from_last(|c| [c, 'b'].into_iter());

        assert_eq!(i.next(), Some('a'));
        assert_eq!(i.next(), Some('b'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn empty() {
        let actual = core::iter::empty()
            .chain_from_last(|_: i32| core::iter::empty())
            .next();

        assert_eq!(actual, None);
    }
}
