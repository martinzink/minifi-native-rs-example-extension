use minifi_native::{MockLogger, MockProcessContext, MockProcessSession, MockProcessSessionFactory};
use super::*;

#[test]
fn test_on_trigger() {
    let mut processor = SimpleSourceProcessor::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context.properties.insert("Content".to_string(), "Hello, World!".to_string());
    let mut session_factory = MockProcessSessionFactory{};

    processor.on_schedule(&context, &mut session_factory);

    {
        let mut session = MockProcessSession::new();
        processor.on_trigger(&context, &mut session);
        assert_eq!(session.transferred_flow_files.get("success").unwrap().content, "Hello, World!")
    }
}