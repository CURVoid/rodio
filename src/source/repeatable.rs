use std::time::Duration;

use crate::{Sample, Source};

use super::Buffered;

/// Internal function that builds a `Repeatable` object.
pub fn repeatable<I>(source: I) -> Repeatable<I>
where
    I: Source,
    I::Item: Sample
{
    let source = source.buffered();
    Repeatable {
        current: source.clone(),
        start: source,
        repeat: false,
    }
}

#[derive(Clone)]
pub struct Repeatable<I>
where
    I: Source,
    I::Item: Sample {
    start: Buffered<I>,
    current: Buffered<I>,
    repeat: bool,
}

impl<I> Repeatable<I>
where
    I: Source,
    I::Item: Sample
{
    /// Starts repeating the sound.
    #[inline]
    pub fn set_repeat(&mut self, value: bool) {
        self.repeat = value;
    }

    /// Returns a reference to the inner source.
    #[inline]
    pub fn inner(&self) -> &Buffered<I> {
        &self.current
    }

    /// Returns a mutable reference to the inner source.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut Buffered<I> {
        &mut self.current
    }

    /// Returns the inner source.
    #[inline]
    pub fn into_inner(self) -> Buffered<I> {
        self.current
    }
}

impl<I> Iterator for Repeatable<I>
where
    I: Source,
    I::Item: Sample,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
      let current = self.current.next();

      if self.repeat && current.is_none() {
          self.current = self.start.clone();
          return self.current.next();
      }

      current
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.current.size_hint()
    }
}

impl<I> Source for Repeatable<I>
where
    I: Source,
    I::Item: Sample,
{
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        self.current.current_frame_len()
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.current.channels()
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.current.sample_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.current.total_duration()
    }
}

unsafe impl<I> Send for Repeatable<I>
where
    I: Source,
    I::Item: Sample {}