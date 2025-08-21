// log-processor/src/lib.rs

use ctor::ctor;
// Import the necessary public types from the `minifi` crate.
use minificpp::{
    Descriptor, Logger, ProcessContext, Processor, ProcessorBridge, Relationship, Session, SessionFactory
};

/// This is the struct that will hold our processor's state.
/// For this simple example, it just holds the logger.
struct SimpleLogProcessor {
    logger: Logger,
}

impl Processor for SimpleLogProcessor {
    fn new(logger: Logger) -> Self {
        Self { logger }
    }

    fn initialize(&mut self, descriptor: &mut Descriptor) {
        descriptor.set_supported_relationships(&[Relationship::new(
            "success",
            "FlowFiles are transferred here after logging",
        )]);
    }

    fn on_trigger(&mut self, _context: &ProcessContext, session: &mut Session) {
        println!("println from onTrigger!");
        self.logger.info("Rust says hello from onTrigger!");


        if let Some(flow_file) = session.get() {
            session.transfer(flow_file, "success");
        }
    }

    fn on_schedule(&mut self, context: &ProcessContext, session_factory: &mut SessionFactory) {
        println!("println from onSchedule!");
        self.logger.info("Rust says hello from onSchedule!");

    }

    fn getName(&self) -> &'static str {
        "SimpleLogProcessor"
    }
}


#[ctor]
#[no_mangle]
fn on_load_register() {
    let bridge = ProcessorBridge::<SimpleLogProcessor>::new(
        "RustProcessors",
        "SimpleLogProcessor",
        "rust::SimpleLogProcessor",
        "A simple processor that logs 'Hello from Rust!' on trigger.",
    );

    // This is the only unsafe call needed, to perform the final registration.
    unsafe {
        minificpp::sys::MinifiRegisterProcessorClass(&bridge.description);
    }
}
