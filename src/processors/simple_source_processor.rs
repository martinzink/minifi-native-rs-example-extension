use ctor::ctor;
use minifi_native::ProcessorInputRequirement::{Forbidden};
use minifi_native::{
    Logger, ProcessContext, Processor, ProcessorBridge, Property, Relationship, Session,
    SessionFactory, StandardPropertyValidator,
};

#[derive(Debug)]
struct SimpleSourceProcessor {
    logger: Logger,
}

impl SimpleSourceProcessor {
    const SUCCESS_RELATIONSHIP: Relationship = Relationship {
        name: "success",
        description: "FlowFiles are transferred here after logging",
    };

    const CONTENT_PROPERTY: Property = Property {
        name: "Content",
        description: "What to write to the flowfile.",
        is_required: false,
        is_sensitive: false,
        supports_expr_lang: false,
        default_value: Some("Something default to write"),
        validator: StandardPropertyValidator::AlwaysValidValidator,
        allowed_values: &[],
        allowed_types: &[],
    };

    const SHOUT_PROPERTY: Property = Property {
        name: "Shouting",
        description: "do you want to shout?",
        is_required: true,
        is_sensitive: false,
        supports_expr_lang: false,
        default_value: Some("false"),
        validator: StandardPropertyValidator::BoolValidator,
        allowed_values: &[],
        allowed_types: &[],
    };
}

impl Processor for SimpleSourceProcessor {
    fn new(logger: Logger) -> Self {
        Self {
            logger,
        }
    }

    fn on_trigger(&mut self, _context: &ProcessContext, session: &mut Session) {
        self.logger
            .trace(format!("on_trigger exit {:?}", self).as_str());

        if let Some(new_ff) = session.create() {
            self.logger.info(format!("Created new flowfile").as_str());
            session.write(&new_ff, "Its just you and me");
            session.transfer(new_ff, Self::SUCCESS_RELATIONSHIP.name);
        }

        self.logger
            .trace(format!("on_trigger exit {:?}", self).as_str());
    }

    fn on_schedule(&mut self, _context: &ProcessContext, _session_factory: &mut SessionFactory) {
        self.logger
            .trace(format!("on_schedule entry {:?}", self).as_str());

        self.logger
            .trace(format!("on_schedule exit {:?}", self).as_str());
    }
}

#[ctor]
#[no_mangle]
fn register_simple_source_processor() {
    let mut my_rust_processor = ProcessorBridge::<SimpleSourceProcessor>::new(
        "rust_extension",
        "mzink.processors.rust.SimpleSourceProcessor",
        "A rust processor that acts as a source.",
    );

    my_rust_processor.is_single_threaded = true;
    my_rust_processor.input_requirement = Forbidden;
    my_rust_processor.supports_dynamic_properties = false;
    my_rust_processor.supports_dynamic_relationships = false;
    my_rust_processor.relationships = vec![SimpleSourceProcessor::SUCCESS_RELATIONSHIP];
    my_rust_processor.properties = vec![
        SimpleSourceProcessor::CONTENT_PROPERTY,
        SimpleSourceProcessor::SHOUT_PROPERTY,
    ];

    my_rust_processor.register_class();
}
