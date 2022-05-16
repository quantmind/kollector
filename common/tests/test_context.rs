use common::Context;

type SimpleContext = Context<()>;

#[test]
fn simple_context() {
    let context = SimpleContext::new("test", None);
    assert_eq!(context.name, "test");
}
