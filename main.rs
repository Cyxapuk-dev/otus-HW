use std::sync::Mutex;
fn main() {}
#[derive(Debug, PartialEq)]
pub enum WriteError {
    NoSpaceLeft,
}

pub struct RingBuffer<const N: usize> {
    inner: Mutex<InnerRingBuffer<N>>,
}

struct InnerRingBuffer<const N: usize> {
    read_idx: usize,
    write_idx: usize,
    data: [u8; N],
    size: usize,
}

impl<const N: usize> RingBuffer<N> {
    pub fn create() -> Self {
        Self {
            inner: Mutex::new(InnerRingBuffer {
                read_idx: 0,
                write_idx: 0,
                data: [0; N],
                size: 0,
            }),
        }
    }

    pub fn write(&self, input: &[u8]) -> Result<usize, WriteError> {
        let mut buf = self.inner.lock().unwrap();

        if buf.size == N {
            return Err(WriteError::NoSpaceLeft);
        }

        let mut written = 0;
        for &byte in input {
            if buf.size == N {
                break;
            }

            let idx = buf.write_idx;
            buf.data[idx] = byte;
            buf.write_idx = (buf.write_idx + 1) % N;
            buf.size += 1;
            written += 1;
        }

        if written == 0 {
            Err(WriteError::NoSpaceLeft)
        } else {
            Ok(written)
        }
    }

    pub fn read(&self, count: usize) -> Option<Vec<u8>> {
        let mut buf = self.inner.lock().unwrap();

        if buf.size == 0 || count == 0 {
            return None;
        }

        let read_count = count.min(buf.size);
        let mut output = Vec::with_capacity(read_count);

        for _ in 0..read_count {
            let idx = buf.read_idx;
            output.push(buf.data[idx]);
            buf.read_idx = (buf.read_idx + 1) % N;
            buf.size -= 1;
        }

        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_ring_buffer_basic() {
        let rb = RingBuffer::<3>::create();
        assert_eq!(rb.write(b"ab"), Ok(2));
        assert_eq!(rb.write(b"cd"), Ok(1));
        assert_eq!(rb.read(1), Some(b"a".to_vec()));
        assert_eq!(rb.write(b"e"), Ok(1));
        assert_eq!(rb.read(2), Some(b"bc".to_vec()));
    }

    #[test]
    fn test_threaded_read_write() {
        let rb = Arc::new(RingBuffer::<100>::create());

        let writer_rb = Arc::clone(&rb);
        let writer = thread::spawn(move || {
            for _ in 0..1000 {
                loop {
                    if writer_rb.write(b"x").is_ok() {
                        break;
                    }
                }
            }
        });

        let reader_rb = Arc::clone(&rb);
        let reader = thread::spawn(move || {
            let mut total_read = 0;
            while total_read < 1000 {
                if let Some(bytes) = reader_rb.read(10) {
                    total_read += bytes.len();
                }
            }
            assert_eq!(total_read, 1000);
        });

        writer.join().unwrap();
        reader.join().unwrap();
    }
}
