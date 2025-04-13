struct RingBuffer<const N: usize> {
    read_idx: usize,
    write_idx: usize,
    data: [u8; N],
    size: usize,
}

impl<const N: usize> RingBuffer<N> {
    fn create() -> Self {
        Self {
            read_idx: 0,
            write_idx: 0,
            data: [0; N],
            size: 0,
        }
    }

    fn write(&mut self, input: &[u8]) -> usize {
        let mut written = 0;
        for &byte in input {
            if self.size == N {
                break; // Буфер заполнен
            }
            self.data[self.write_idx] = byte;
            self.write_idx = (self.write_idx + 1) % N;
            self.size += 1;
            written += 1;
        }
        written
    }

    fn read(&mut self, count: usize) -> Vec<u8> {
        let read_count = count.min(self.size);
        let mut output = Vec::with_capacity(read_count);

        for _ in 0..read_count {
            output.push(self.data[self.read_idx]);
            self.read_idx = (self.read_idx + 1) % N;
        }
        self.size -= read_count;
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer() {
        let mut rb = RingBuffer::<3>::create();
        assert_eq!(rb.write(b"ab"), 2);
        assert_eq!(rb.write(b"cd"), 1);
        assert_eq!(rb.read(1), b"a".to_vec());
        assert_eq!(rb.write(b"e"), 1);
        assert_eq!(rb.read(2), b"bc".to_vec());
    }

    #[test]
    fn test_wrap_around() {
        let mut rb = RingBuffer::<3>::create();
        assert_eq!(rb.write(b"abc"), 3);
        assert_eq!(rb.read(2), b"ab".to_vec());
        assert_eq!(rb.write(b"de"), 2);
        assert_eq!(rb.read(3), b"cde".to_vec());
    }
}

fn main() {}