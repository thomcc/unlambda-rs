#[cfg(feature = "arc")]
type InnerP<T> = std::sync::Arc<T>;
#[cfg(not(feature = "arc"))]
type InnerP<T> = std::rc::Rc<T>;

/// This type is a quick hack to paper over a mistake where apparently I exposed
/// Arc vs Rc based on a cfg(feature = "arc") setting. It was very easy to write
/// code that breaks when that feature changes, so now there's a hacky/minimal
/// wrapper.
///
/// Don't ask me about the 1-char name, I wrote this like a year ago.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct P<T>(InnerP<T>);

impl<T> std::ops::Deref for P<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<T> P<T> {
    #[inline]
    pub fn new(v: T) -> Self {
        Self(InnerP::new(v))
    }
}

#[inline]
pub(crate) fn p<T>(v: T) -> P<T> {
    P::new(v)
}
