use std::string::ToString;
use ctor::ctor;
use minifi_native::{
    Descriptor, Logger, ProcessContext, Processor, ProcessorBridge, Property, StandardPropertyValidator, Relationship,
    Session, SessionFactory,
};

struct SimpleLogProcessor {
    logger: Logger,
    what_to_log: Option<String>,
    what_to_log_property_name: String,
}

impl SimpleLogProcessor {
    const SUCCESS_RELATIONSHIP: Relationship = Relationship::new(
        "success",
        "FlowFiles are transferred here after logging",
    );

}

impl Processor for SimpleLogProcessor {
    fn new(logger: Logger) -> Self {
        Self { logger, what_to_log: None, what_to_log_property_name: "text".to_string() }
    }

    fn initialize(&mut self, descriptor: &mut Descriptor) {
        descriptor.set_supported_relationships(&[SimpleLogProcessor::SUCCESS_RELATIONSHIP]);
        let what_to_log_property: Property = Property {
            name: self.what_to_log_property_name.clone(),
            description: "what to log".to_string(),
            is_required: false,
            is_sensitive: false,
            supports_expr_lang: false,
            default_value: Some("Default text to log.".to_string()),
            validator: StandardPropertyValidator::AlwaysValidValidator,
        };
        descriptor.set_supported_properties(&[what_to_log_property])
    }

    fn on_trigger(&mut self, _context: &ProcessContext, session: &mut Session) {
        self.logger.info(format!("rusty on_trigger: {:?}", self.what_to_log).as_str());

        if let Some(flow_file) = session.get() {
            session.transfer(flow_file, "success");
        }
    }

    fn on_schedule(&mut self, context: &ProcessContext, _session_factory: &mut SessionFactory) {
        self.what_to_log = context.get_property(self.what_to_log_property_name.as_str(), None);
        self.logger.info(format!("rusty on_schedule: {:?}", self.what_to_log).as_str());
    }

    fn get_name(&self) -> &'static str {
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
        "A simple processor that logs some text during onSchedule and onTrigger.",
    );

    unsafe {
        minifi_native::sys::MinifiRegisterProcessorClass(&bridge.description);
    }
}
