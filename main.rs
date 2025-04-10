#[derive(Debug, PartialEq)]
pub enum WriteError {
    NoSpaceLeft,
}

pub struct RingBuffer<const N: usize> {
    read_idx: usize,
    write_idx: usize,
    data: [u8; N],
    size: usize,
}

impl<const N: usize> RingBuffer<N> {
    pub fn create() -> Self {
        Self {
            read_idx: 0,
            write_idx: 0,
            data: [0; N],
            size: 0,
        }
    }

    pub fn write(&mut self, input: &[u8]) -> Result<usize, WriteError> {
        if self.size == N {
            return Err(WriteError::NoSpaceLeft);
        }

        let mut written = 0;
        for &byte in input {
            if self.size == N {
                break;
            }
            self.data[self.write_idx] = byte;
            self.write_idx = (self.write_idx + 1) % N;
            self.size += 1;
            written += 1;
        }

        if written == 0 {
            Err(WriteError::NoSpaceLeft)
        } else {
            Ok(written)
        }
    }

    pub fn read(&mut self, count: usize) -> Option<Vec<u8>> {
        if self.size == 0 || count == 0 {
            return None;
        }

        let read_count = count.min(self.size);
        let mut output = Vec::with_capacity(read_count);

        for _ in 0..read_count {
            output.push(self.data[self.read_idx]);
            self.read_idx = (self.read_idx + 1) % N;
            self.size -= 1;
        }

        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer() {
        let mut rb = RingBuffer::<3>::create();
        assert_eq!(rb.write(b"ab"), Ok(2));
        assert_eq!(rb.write(b"cd"), Ok(1)); // только "c" влезла
        assert_eq!(rb.read(1), Some(b"a".to_vec()));
        assert_eq!(rb.write(b"e"), Ok(1));
        assert_eq!(rb.read(2), Some(b"bc".to_vec()));
    }

    #[test]
    fn test_wrap_around() {
        let mut rb = RingBuffer::<3>::create();
        assert_eq!(rb.write(b"abc"), Ok(3));
        assert_eq!(rb.read(2), Some(b"ab".to_vec()));
        assert_eq!(rb.write(b"de"), Ok(2));
        assert_eq!(rb.read(3), Some(b"cde".to_vec()));
    }

    #[test]
    fn test_no_space_left() {
        let mut rb = RingBuffer::<2>::create();
        assert_eq!(rb.write(b"x"), Ok(1));
        assert_eq!(rb.write(b"y"), Ok(1));
        assert_eq!(rb.write(b"z"), Err(WriteError::NoSpaceLeft));
    }

    #[test]
    fn test_empty_read() {
        let mut rb = RingBuffer::<2>::create();
        assert_eq!(rb.read(1), None);
        rb.write(b"a").unwrap();
        rb.read(1).unwrap();
        assert_eq!(rb.read(1), None);
    }
}

fn main() {}