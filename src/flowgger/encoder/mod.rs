mod capnp_encoder;
mod gelf_encoder;
mod ltsv_encoder;

pub use self::capnp_encoder::CapnpEncoder;
pub use self::gelf_encoder::GelfEncoder;
pub use self::ltsv_encoder::LTSVEncoder;

use crate::flowgger::record::Record;

pub trait CloneBoxedEncoder {
    fn clone_boxed<'a>(&self) -> Box<dyn Encoder + Send + 'a>
    where
        Self: 'a;
}

impl<T: Encoder + Clone + Send> CloneBoxedEncoder for T {
    fn clone_boxed<'a>(&self) -> Box<dyn Encoder + Send + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Encoder> {
    fn clone(&self) -> Box<dyn Encoder> {
        self.clone_boxed()
    }
}

pub trait Encoder: CloneBoxedEncoder {
    fn encode(&self, record: Record) -> Result<Vec<u8>, &'static str>;
}
