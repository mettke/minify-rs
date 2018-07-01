use core::fmt;
use json::json_minifier::JsonMinifier;

pub struct MultiFilter<I: Iterator + fmt::Debug, P> {
    minifier: JsonMinifier,
    iter: I,
    predicate: P,
    initialized: bool,
    item1: Option<I::Item>,
}

impl<I: Iterator + fmt::Debug, P> MultiFilter<I, P> {
    #[inline]
    pub fn new(iter: I, predicate: P) -> Self {
        MultiFilter {
            minifier: JsonMinifier::new(),
            iter,
            predicate,
            initialized: false,
            item1: None,
        }
    }
}

impl<I: Iterator + fmt::Debug, P> fmt::Debug for MultiFilter<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Filter").field("iter", &self.iter).finish()
    }
}

impl<I: Iterator + fmt::Debug, P> Iterator for MultiFilter<I, P>
where
    P: FnMut(&mut JsonMinifier, &I::Item, Option<&I::Item>) -> bool,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if !self.initialized {
            self.item1 = self.iter.next();
            self.initialized = true;
        }

        while let Some(item) = self.item1.take() {
            self.item1 = self.iter.next();

            if (self.predicate)(&mut self.minifier, &item, self.item1.as_ref()) {
                return Some(item);
            }
        }
        None
    }
}
