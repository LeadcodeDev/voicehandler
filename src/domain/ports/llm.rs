pub trait Llm: Send + Sync + 'static {
    fn pipe();
}
