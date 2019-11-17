use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RunSignal(Arc<AtomicBool>);

impl RunSignal {

    pub fn new() -> RunSignal {
        RunSignal(Arc::new(AtomicBool::new(true)))
    }

    pub fn is_running(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }

    pub fn stop(&mut self) {
        self.0.store(false, Ordering::Relaxed)
    }
}
