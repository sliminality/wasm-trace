use ring_buffer::RingBuffer;

pub static LOG_CALL: &str = "__log_call";
pub static EXPOSE_TRACER: &str = "__expose_tracer";
pub static EXPOSE_TRACER_LEN: &str = "__expose_tracer_len";

static RING_BUFFER_ENTRIES: usize = 1024;

#[repr(i32)]
#[derive(Debug)]
pub enum EntryKind {
    FunctionCall = 0,
    FunctionReturnVoid = 1,
    FunctionReturnValue = 2,
}

/// Wrapper around the ring buffer for recording function calls.
#[derive(Debug)]
pub struct Tracer(RingBuffer<i32>);

impl Tracer {
    pub fn new() -> Self {
        Tracer(RingBuffer::new(RING_BUFFER_ENTRIES * 2))
    }

    pub fn log(&mut self, kind: i32, data: i32) {
        self.0.enqueue(kind as i32);
        self.0.enqueue(data);
    }

    pub fn as_ptr(&self) -> *const i32 {
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
        pub fn __log_call(id: i32, data: i32) {
            TRACER.lock().unwrap().log(id, data);
        }

        #[allow(private_no_mangle_fns)]
        #[no_mangle]
        pub fn __expose_tracer() -> *const i32 {
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
    use itertools::Itertools;
    use super::{Tracer, EntryKind};

    #[test]
    fn get_ptr() {
        let mut tracer = Tracer::new();
        let kinds = vec![EntryKind::FunctionCall as i32,
                         EntryKind::FunctionCall as i32,
                         EntryKind::FunctionCall as i32];
        let values = vec![4, 1, 2];
        for (&kind, &x) in kinds.clone().iter().zip(values.clone().iter()) {
            tracer.log(kind, x);
        }

        let ptr = tracer.as_ptr();
        let len = tracer.len();
        let mut expected_values = kinds.iter().interleave(values.iter());

        unsafe {
            for i in 0..len {
                let &expected = expected_values.next().unwrap();
                assert_eq!(*ptr.offset(i as isize), expected);
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
