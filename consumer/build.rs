use std::sync::atomic::Ordering;
use producer::{
    PLAIN,
    SIMPLE,
    PLAIN_DECLARATIVE,
    SIMPLE_DECLARATIVE,
    PRIORITY,
    PRIORITY_DECLARATIVE,
};

fn main() {
    let plain = PLAIN.load(Ordering::Relaxed);
    let simple = SIMPLE.load(Ordering::Relaxed);
    let plain_declarative = PLAIN_DECLARATIVE.load(Ordering::Relaxed);
    let simple_declarative = SIMPLE_DECLARATIVE.load(Ordering::Relaxed);
    let priority = PRIORITY.load(Ordering::Relaxed);
    let priority_declarative = PRIORITY_DECLARATIVE.load(Ordering::Relaxed);

    println!("cargo:warning=plain                is {plain}");
    println!("cargo:warning=simple               is {simple}");
    println!("cargo:warning=plain_declarative    is {plain_declarative}");
    println!("cargo:warning=simple_declarative   is {simple_declarative}");
    println!("cargo:warning=priority             is {priority}");
    println!("cargo:warning=priority_declarative is {priority_declarative}");
}
