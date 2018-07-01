use std::fmt;
use std::mem;

pub struct MultiFilter<I: Iterator, P, M> {
    minifier: M,
    iter: I,
    predicate: P,
    initialized: bool,
    item1: Option<I::Item>,
    item2: Option<I::Item>,
    item3: Option<I::Item>,
    item4: Option<I::Item>,
}

impl<I: Iterator, P, M: Default> MultiFilter<I, P, M> {
    #[inline]
    pub fn new(iter: I, predicate: P) -> Self {
        MultiFilter {
            minifier: M::default(),
            iter,
            predicate,
            initialized: false,
            item1: None,
            item2: None,
            item3: None,
            item4: None,
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

impl<I, P, M> Iterator for MultiFilter<I, P, M>
where
    I: Iterator,
    P: FnMut(
        &mut M,
        &I::Item,
        Option<&I::Item>,
        Option<&I::Item>,
        Option<&I::Item>,
        Option<&I::Item>,
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
            self.initialized = true;
        }

        while let Some(item) = self.item1.take() {
            mem::swap(&mut self.item1, &mut self.item2);
            mem::swap(&mut self.item2, &mut self.item3);
            mem::swap(&mut self.item3, &mut self.item4);
            self.item4 = self.iter.next();

            if (self.predicate)(
                &mut self.minifier,
                &item,
                self.item1.as_ref(),
                self.item2.as_ref(),
                self.item3.as_ref(),
                self.item4.as_ref(),
            ) {
                return Some(item);
            }
        }
        None
    }
}
