use ctor::ctor;
use minifi_native::ProcessorInputRequirement::Allowed;
use minifi_native::{
    LogLevel, Logger, ProcessContext, Processor, ProcessorBridge, Property, Relationship, Session,
    SessionFactory, StandardPropertyValidator,
};
use strum::VariantNames;

#[derive(Debug)]
struct SimpleLogProcessor {
    logger: Logger,
    log_level: LogLevel,
}

impl SimpleLogProcessor {
    const SUCCESS_RELATIONSHIP: Relationship = Relationship {
        name: "success",
        description: "FlowFiles are transferred here after logging",
    };

    const LOG_LEVEL: Property = Property {
        name: "Log Level",
        description: "The level of logging.",
        is_required: true,
        is_sensitive: false,
        supports_expr_lang: false,
        default_value: Some("info"),
        validator: StandardPropertyValidator::AlwaysValidValidator,
        allowed_values: &LogLevel::VARIANTS,
        allowed_types: &[],
    };
}

impl Processor for SimpleLogProcessor {
    fn new(logger: Logger) -> Self {
        Self {
            logger,
            log_level: LogLevel::Info,
        }
    }

    fn on_trigger(&mut self, _context: &ProcessContext, session: &mut Session) {
        self.logger
            .trace(format!("on_trigger entry {:?}", self).as_str());


         if let Some(input_ff) = session.get() {
             self.logger.trace(format!("Got flowfile").as_str());
             if let Some(content) = session.read(&input_ff) {
                 self.logger.log(self.log_level, content.as_str());
             }
             session.transfer(input_ff, Self::SUCCESS_RELATIONSHIP.name);

         }

        self.logger
            .trace(format!("on_trigger exit {:?}", self).as_str());
    }

    fn on_schedule(&mut self, context: &ProcessContext, _session_factory: &mut SessionFactory) {
        self.logger
            .trace(format!("on_schedule entry {:?}", self).as_str());

        self.log_level = context
            .get_property(SimpleLogProcessor::LOG_LEVEL.name, None)
            .and_then(|s| s.parse::<LogLevel>().ok())
            .unwrap_or(LogLevel::Info);

        self.logger
            .trace(format!("on_schedule exit {:?}", self).as_str());
    }
}

#[ctor]
#[no_mangle]
fn register_simple_log_processor() {
    let mut my_rust_processor = ProcessorBridge::<SimpleLogProcessor>::new(
        "rust_extension",
        "mzink.processors.rust.SimpleLogProcessor",
        "A rust processor that tests the upcoming C API, trying out most of the features",
    );

    my_rust_processor.is_single_threaded = true;
    my_rust_processor.input_requirement = Allowed;
    my_rust_processor.supports_dynamic_properties = false;
    my_rust_processor.supports_dynamic_relationships = false;
    my_rust_processor.relationships = vec![SimpleLogProcessor::SUCCESS_RELATIONSHIP];
    my_rust_processor.properties = vec![
        SimpleLogProcessor::LOG_LEVEL,
    ];

    my_rust_processor.register_class();
}
