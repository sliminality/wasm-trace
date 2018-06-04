use ring_buffer::RingBuffer;

pub static LOG_CALL: &str = "__log_call";
pub static EXPOSE_TRACER: &str = "__expose_tracer";
pub static EXPOSE_TRACER_LEN: &str = "__expose_tracer_len";

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

#[macro_export]
/// Provides dependencies for the Tracer.
/// Designed to be imported into a user's module at the root level.
macro_rules! tracer_dependencies {
    () => {
        #[macro_use] extern crate lazy_static;
        use ::std::sync::Mutex;
    }
}

#[macro_export]
/// Bootstraps a Tracer into the module root, allowing our reinstrumentation to
/// write to the ring buffer.
macro_rules! tracer_bootstrap {
    () => {
        lazy_static! {
            static ref TRACER: Mutex<Tracer> = Mutex::new(Tracer::new());
        }

        #[allow(private_no_mangle_fns)]
        #[no_mangle]
        pub fn __log_call(id: u32) {
            TRACER.lock().unwrap().log(id);
        }

        #[allow(private_no_mangle_fns)]
        #[no_mangle]
        pub fn __expose_tracer() -> *const u32 {
            TRACER.lock().unwrap().as_ptr()
        }

        #[allow(private_no_mangle_fns)]
        #[no_mangle]
        pub fn __expose_tracer_len() -> u32 {
            TRACER.lock().unwrap().len() as u32
        }
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

    #[test]
    fn bootstrap() {
        use std::sync::Mutex;
        tracer_bootstrap!();
        assert_eq!(__expose_tracer_len(), 0);
    }
}
