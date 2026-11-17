//! SigmaOS-mint-update-notifier — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_mint_update_notifier_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-mint-update-notifier: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-mint-update-notifier: ok");
}
