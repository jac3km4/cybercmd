use crate::path::Path;

pub trait BasePathExt: private::Sealed {
    fn ancestors(&self) -> BasePathExtAncestors<'_>;
}

impl BasePathExt for Path {
    fn ancestors(&self) -> BasePathExtAncestors<'_> {
        BasePathExtAncestors { next: Some(self) }
    }
}

#[derive(Copy, Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct BasePathExtAncestors<'a> {
    next: Option<&'a Path>,
}

impl<'a> Iterator for BasePathExtAncestors<'a> {
    type Item = &'a Path;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        self.next = next.and_then(Path::parent_unchecked);
        next
    }
}

impl std::iter::FusedIterator for BasePathExtAncestors<'_> {}

mod private {
    pub trait Sealed {}
    impl Sealed for crate::path::Path {}
}
