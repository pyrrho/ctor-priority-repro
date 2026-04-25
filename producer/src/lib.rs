use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use ctor::ctor;

pub static PLAIN: AtomicBool = AtomicBool::new(false);
pub static SIMPLE: AtomicBool = AtomicBool::new(false);
pub static PLAIN_DECLARATIVE: AtomicBool = AtomicBool::new(false);
pub static SIMPLE_DECLARATIVE: AtomicBool = AtomicBool::new(false);

pub static PRIORITY: AtomicBool = AtomicBool::new(false);
pub static PRIORITY_DECLARATIVE: AtomicBool = AtomicBool::new(false);

#[ctor]
unsafe fn register_plain() {
    PLAIN.store(true, Ordering::SeqCst);
}
#[ctor(unsafe)]
fn register_simple() {
    SIMPLE.store(true, Ordering::SeqCst);
}
ctor::declarative::ctor! {
    #[ctor]
    unsafe fn register_plain_declarative() {
        PLAIN_DECLARATIVE.store(true, Ordering::SeqCst);
    }
}
ctor::declarative::ctor! {
    #[ctor(unsafe)]
    fn register_simple_declarative() {
        SIMPLE_DECLARATIVE.store(true, Ordering::SeqCst);
    }
}

#[ctor(unsafe, priority = 100)]
fn register_priority() {
    PRIORITY.store(true, Ordering::SeqCst);
}
ctor::declarative::ctor! {
    #[ctor(unsafe, priority = 100)]
    fn register_priority_declarative() {
        PRIORITY_DECLARATIVE.store(true, Ordering::SeqCst);
    }
}
