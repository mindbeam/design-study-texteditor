use std::ops::{RangeBounds, RangeFrom};
pub struct MutStr<'a> {
    string: &'a mut String,
    start: usize,
    end: usize,
}

impl<'a> MutStr<'a> {
    pub fn slice<T>(string: &'a mut String, range: T) -> Self
    where
        T: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            std::ops::Bound::Included(i) => *i,
            std::ops::Bound::Excluded(i) => *i + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(i) => string.len().min(*i),
            std::ops::Bound::Excluded(i) => string.len().min(*i - 1),
            std::ops::Bound::Unbounded => string.len(),
        };

        Self { start, end, string }
    }
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    pub fn insert_str(&mut self, idx: usize, string: &str) {
        let i = self.start + idx;
        if i > self.end {
            panic!("insert_str out of bounds");
        }

        self.string.insert_str(i, string);
        self.end += string.len();
    }
    pub fn remove(&mut self, idx: usize) {
        let i = self.start + idx;
        if i > self.end {
            panic!("remove out of bounds");
        }

        if i == self.end {
            self.string.pop();
        } else {
            self.string.remove(i);
        }
        self.end -= 1;
    }
}

impl<'a> std::ops::Index<std::ops::RangeFull> for MutStr<'a> {
    type Output = str;

    #[inline]
    fn index(&self, _index: std::ops::RangeFull) -> &str {
        &self.string[self.start..self.end]
    }
}

impl<'a> PartialEq<String> for MutStr<'a> {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(&self[..], &other[..])
    }
    #[inline]
    fn ne(&self, other: &String) -> bool {
        PartialEq::ne(&self[..], &other[..])
    }
}

#[cfg(test)]
mod test {
    use super::MutStr;

    #[test]
    fn slice() {
        let mut foo = "PizzaPizza".to_string();
        let mut bar = MutStr::slice(&mut foo, 0..6);
        assert_eq!(&bar[..], "Pizza");
        bar.insert_str(5, "Saliad");
        bar.remove(8);
        assert_eq!(foo, "PizzaSaladPizza");
    }
    #[test]
    #[should_panic(expected = "insert_str out of bounds")]
    fn bad_insert() {
        let mut foo = "PizzaPizza".to_string();
        let mut bar = MutStr::slice(&mut foo, 0..6);
        assert_eq!(&bar[..], "Pizza");
        bar.insert_str(6, "Salad");
    }
    #[test]
    #[should_panic(expected = "remove out of bounds")]
    fn bad_remove() {
        let mut foo = "PizzaPizza".to_string();
        let mut bar = MutStr::slice(&mut foo, 0..6);
        assert_eq!(&bar[..], "Pizza");
        bar.remove(6);
    }
}
