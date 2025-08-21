// log-processor/src/lib.rs

// Import the necessary public types from the `minifi` crate.
use minificpp::{
    sys::MinifiRelationship, // We still need the raw C struct for relationships
    Descriptor,
    Logger,
    ProcessContext,
    Processor,
    ProcessorBridge,
    Session,
};

/// This is the struct that will hold our processor's state.
/// For this simple example, it just holds the logger.
struct SimpleLogProcessor {
    logger: Logger,
}

/// Here we implement the safe `Processor` trait for our struct.
impl Processor for SimpleLogProcessor {
    fn new(logger: Logger) -> Self {
        Self { logger }
    }

    fn initialize(&mut self, descriptor: &mut Descriptor) {
        // Since we need to pass any incoming flowfiles through,
        // we must define a "success" relationship.
        let success_rel = MinifiRelationship {
            name: minificpp::sys::MinifiStringView {
                data: "success".as_ptr() as *const i8,
                length: "success".len() as u32,
            },
            description: minificpp::sys::MinifiStringView {
                data: "FlowFiles are transferred here after logging".as_ptr() as *const i8,
                length: "FlowFiles are transferred here after logging".len() as u32,
            },
        };
        descriptor.set_supported_relationships(&[success_rel]);
    }

    fn on_trigger(&mut self, _context: &ProcessContext, session: &mut Session) {
        // This is the core logic of our processor.
        self.logger.info("Hello from Rust!");

        // It's good practice to handle any incoming flowfile, even if we don't
        // inspect it. Here, we just transfer it to "success" to prevent data loss.
        if let Some(flow_file) = session.get() {
            session.transfer(flow_file, "success");
        }
    }
}

/// This is the main entry point that the MiNiFi C++ agent will call.
/// It creates the bridge and registers our processor.
#[no_mangle]
pub extern "C" fn MinifiRegister() {
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
