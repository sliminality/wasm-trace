/// Wrapper around the ring buffer for recording function calls.
#[derive(Debug)]
pub struct Tracer(RingBuffer<u32>);

impl Tracer {
    pub fn new() -> Self {
        Tracer(RingBuffer::new(1024))
    }

    pub fn log(&mut self, data: u32) {
        self.0.enqueue(data);
    }

    pub fn as_ptr(&self) -> *const u32 {
        self.0.as_slice().as_ptr()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod test_tracer {
    use super::Tracer;

    #[test]
    fn get_ptr() {
        let mut tracer = Tracer::new();
        let values: [u32; 3] = [4, 1, 2];
        for &x in values.iter() {
            tracer.log(x);
        }
        let ptr = tracer.as_ptr();
        let len = tracer.len();
        unsafe {
            for i in 0..len {
                assert_eq!(*ptr.offset(i as isize), values[i]);
            }
        }
    }
}
