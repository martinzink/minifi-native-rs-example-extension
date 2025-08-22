use std::string::ToString;
use ctor::ctor;
use minifi_native::{
    Descriptor, Logger, ProcessContext, Processor, ProcessorBridge, Property, StandardPropertyValidator, Relationship,
    Session, SessionFactory,
};

#[derive(Debug)]
struct SimpleLogProcessor {
    logger: Logger,
    what_to_log: Option<String>,
    should_shout: bool,
}

impl SimpleLogProcessor {
    const SUCCESS_RELATIONSHIP: Relationship = Relationship::new(
        "success",
        "FlowFiles are transferred here after logging",
    );

    const WHAT_TO_LOG_PROPERTY: Property = Property {
        name: "text",
        description: "what to log",
        is_required: false,
        is_sensitive: false,
        supports_expr_lang: false,
        default_value: Some("Default text to log."),
        validator: StandardPropertyValidator::AlwaysValidValidator,
    };

    const SHOUT_PROPERTY: Property = Property {
        name: "Shouting",
        description: "do you want to shout?",
        is_required: true,
        is_sensitive: false,
        supports_expr_lang: false,
        default_value: Some("false"),
        validator: StandardPropertyValidator::BoolValidator,
    };
}

impl Processor for SimpleLogProcessor {
    fn new(logger: Logger) -> Self {
        Self { logger, what_to_log: None, should_shout: false }
    }

    fn initialize(&mut self, descriptor: &mut Descriptor) {
        descriptor.set_supported_relationships(&[SimpleLogProcessor::SUCCESS_RELATIONSHIP]);
        descriptor.set_supported_properties(&[SimpleLogProcessor::WHAT_TO_LOG_PROPERTY, SimpleLogProcessor::SHOUT_PROPERTY]);
    }

    fn on_trigger(&mut self, _context: &ProcessContext, session: &mut Session) {
        let text_ref = self.what_to_log.as_ref().unwrap_or(&"".to_string()).clone();
        let modified_text = if self.should_shout { text_ref.to_uppercase() } else { text_ref };
        self.logger.info(format!("rusty on_trigger: {}", modified_text.as_str()).as_str());

//        if let Some(flow_file) = session.get() {
//            session.transfer(flow_file, "success");
//        }
    }

    fn on_schedule(&mut self, context: &ProcessContext, _session_factory: &mut SessionFactory) {
        self.logger.info(format!("on_schedule entry {:?}", self).as_str());

        self.what_to_log = context.get_property(SimpleLogProcessor::WHAT_TO_LOG_PROPERTY.name, None);
        self.should_shout = context.get_property(SimpleLogProcessor::SHOUT_PROPERTY.name, None).and_then(|s| s.parse::<bool>().ok()).unwrap_or(false);

        self.logger.info(format!("on_schedule exit {:?}", self).as_str());
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
