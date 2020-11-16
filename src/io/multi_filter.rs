use std::{fmt, mem};

pub struct MultiFilter<I: Iterator, P, M> {
    minifier: M,
    iter: I,
    predicate: P,
    initialized: bool,
    item1: Option<I::Item>,
    item2: Option<I::Item>,
    item3: Option<I::Item>,
    item4: Option<I::Item>,
    item5: Option<I::Item>,
}

impl<I: Iterator, P, M: Default> MultiFilter<I, P, M> {
    #[inline]
    pub fn new(iter: I, predicate: P) -> Self {
        Self {
            minifier: M::default(),
            iter,
            predicate,
            initialized: false,
            item1: None,
            item2: None,
            item3: None,
            item4: None,
            item5: None,
        }
    }
}

impl<I: Iterator + fmt::Debug, P, M> fmt::Debug for MultiFilter<I, P, M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Filter")
            .field("iter", &self.iter)
            .field("initialized", &self.initialized)
            .finish()
    }
}

impl<I, It, P, M> Iterator for MultiFilter<I, P, M>
where
    I: Iterator<Item = It>,
    It: Copy,
    P: FnMut(
        &mut M,
        I::Item,
        Option<I::Item>,
        Option<I::Item>,
        Option<I::Item>,
        Option<I::Item>,
        Option<I::Item>,
    ) -> bool,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if !self.initialized {
            self.item1 = self.iter.next();
            self.item2 = self.iter.next();
            self.item3 = self.iter.next();
            self.item4 = self.iter.next();
            self.item5 = self.iter.next();
            self.initialized = true;
        }

        while let Some(item) = self.item1.take() {
            mem::swap(&mut self.item1, &mut self.item2);
            mem::swap(&mut self.item2, &mut self.item3);
            mem::swap(&mut self.item3, &mut self.item4);
            mem::swap(&mut self.item4, &mut self.item5);
            self.item5 = self.iter.next();

            if (self.predicate)(
                &mut self.minifier,
                item,
                self.item1,
                self.item2,
                self.item3,
                self.item4,
                self.item5,
            ) {
                return Some(item);
            }
        }
        None
    }
}
