use ctor::ctor;
use minifi_native::ProcessorInputRequirement::Forbidden;
use minifi_native::{
    CffiLogger, Logger, ProcessContext, ProcessSession, ProcessSessionFactory, Processor,
    ProcessorBridge, Property, Relationship, StandardPropertyValidator,
};

#[derive(Debug)]
struct SimpleSourceProcessor<L: Logger> {
    logger: L,
}

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

impl<L: Logger> Processor<L> for SimpleSourceProcessor<L> {
    fn new(logger: L) -> Self {
        Self { logger }
    }

    fn on_trigger<P, S>(&mut self, _context: &P, session: &mut S)
    where
        P: ProcessContext,
        S: ProcessSession,
    {
        self.logger
            .trace(format!("on_trigger exit {:?}", self).as_str());

        if let Some(new_ff) = session.create() {
            self.logger.info(format!("Created new flowfile").as_str());
            session.write(&new_ff, "Its just you and me");
            session.transfer(new_ff, SUCCESS_RELATIONSHIP.name);
        }

        self.logger
            .trace(format!("on_trigger exit {:?}", self).as_str());
    }

    fn on_schedule<P, F>(&mut self, _context: &P, _session_factory: &mut F)
    where
        P: ProcessContext,
        F: ProcessSessionFactory,
    {
        self.logger
            .trace(format!("on_schedule entry {:?}", self).as_str());

        self.logger
            .trace(format!("on_schedule exit {:?}", self).as_str());
    }
}

#[ctor]
#[no_mangle]
fn register_simple_source_processor() {
    let mut my_rust_processor = ProcessorBridge::<SimpleSourceProcessor<CffiLogger>>::new(
        "rust_extension",
        "mzink.processors.rust.SimpleSourceProcessor",
        "A rust processor that acts as a source.",
    );

    my_rust_processor.is_single_threaded = true;
    my_rust_processor.input_requirement = Forbidden;
    my_rust_processor.supports_dynamic_properties = false;
    my_rust_processor.supports_dynamic_relationships = false;
    my_rust_processor.relationships = vec![SUCCESS_RELATIONSHIP];
    my_rust_processor.properties = vec![CONTENT_PROPERTY, SHOUT_PROPERTY];

    my_rust_processor.register_class();
}
