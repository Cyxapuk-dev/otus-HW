fn main() {}
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

    fn read(&mut self, count: usize) -> &[u8] {
        let read_count = count.min(self.size);
        let start = self.read_idx;
        let end = (self.read_idx + read_count) % N;

        self.read_idx = end;
        self.size -= read_count;

        if start < end {
            &self.data[start..end]
        } else {
            &self.data[start..]
        }
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
        assert_eq!(rb.read(1), b"a");
        assert_eq!(rb.write(b"e"), 1);
        assert_eq!(rb.read(2), b"bc");
    }
}
