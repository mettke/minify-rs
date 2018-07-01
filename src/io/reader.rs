use io::multi_filter::MultiFilter;
use io::unstable::{Chars, CharsError};
use std::fmt;
use std::fmt::Formatter;
use std::io::{Read, Result};
use std::iter::FilterMap;
use std::iter::Iterator;
use std::result;

pub struct InternalReader<R: Read, P, M> {
    iter: MultiFilter<
        FilterMap<Chars<R>, fn(result::Result<char, CharsError>) -> Option<char>>,
        P,
        M,
    >,
    bytes: Option<Vec<u8>>,
    pos_bytes: usize,
}

impl<R: Read, P, M: Default> InternalReader<R, P, M> {
    pub fn new(inner_reader: R, predicate: P) -> Self {
        InternalReader {
            iter: MultiFilter::new(
                Chars {
                    inner: inner_reader,
                }.filter_map(Self::filter_map_result_error),
                predicate,
            ),
            bytes: None,
            pos_bytes: 0,
        }
    }

    fn filter_map_result_error(result: result::Result<char, CharsError>) -> Option<char> {
        match result {
            Ok(e) => Some(e),
            _ => None,
        }
    }

    fn handle_bytes(
        &mut self,
        bytes: Vec<u8>,
        bytes_start: usize,
        buf: &mut [u8],
        buf_start: usize,
    ) -> usize {
        let mut pos_bytes = bytes_start;
        let mut pos_buf = buf_start;

        for pos in buf_start..buf.len() {
            if pos_bytes < bytes.len() {
                buf[pos] = bytes[pos_bytes];
                pos_bytes += 1;
                pos_buf += 1;
            } else {
                break;
            }
        }
        if pos_bytes < bytes.len() {
            self.pos_bytes = pos_bytes;
            self.bytes = Some(bytes);
        }
        pos_buf
    }
}

impl<R: Read + fmt::Debug, P, M> fmt::Debug for InternalReader<R, P, M> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("InternalReader")
            .field("iter", &self.iter)
            .field("bytes", &self.bytes)
            .field("pos_bytes", &self.pos_bytes)
            .finish()
    }
}

impl<R, P, M> Read for InternalReader<R, P, M>
where
    R: Read,
    P: FnMut(&mut M, &char, Option<&char>, Option<&char>, Option<&char>, Option<&char>) -> bool,
    M: Default,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut pos_buffer = 0;

        if let Some(item) = self.bytes.take() {
            let pos_bytes = self.pos_bytes;
            pos_buffer = self.handle_bytes(item, pos_bytes, buf, 0);
            if pos_buffer >= buf.len() {
                return Ok(pos_buffer);
            }
        }

        while let Some(item) = self.iter.next() {
            let bytes = item.to_string().into_bytes();
            pos_buffer = self.handle_bytes(bytes, 0, buf, pos_buffer);
            if pos_buffer >= buf.len() {
                break;
            }
        }
        Ok(pos_buffer)
    }
}
