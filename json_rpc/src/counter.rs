pub mod utils {
    use std::cell::Cell;

    #[derive(Debug, Default)]
    pub(crate) struct Counter(Cell<usize>);

    impl Counter {
        pub(crate) fn next(&self) -> usize {
            let n = self.0.get();
            self.0.set(n + 1);
            n + 1
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_id() {
            let id_counter = Counter::default();
            assert_eq!(id_counter.next(), 1);
            assert_eq!(id_counter.next(), 2);
        }
    }
}

/*
/// BufferIds uniquely identify open buffers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct BufferId(pub(crate) usize);

impl fmt::Display for BufferId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "buffer-id-{}", self.0)
    }
}

impl BufferId {
    pub fn new(val: usize) -> Self {
        BufferId(val)
    }
}
*/
