/// Should only be called once... especially relevant for testing!
pub fn init_subscriber() {
    // initialize tracing
    tracing_subscriber::fmt::init();
}
