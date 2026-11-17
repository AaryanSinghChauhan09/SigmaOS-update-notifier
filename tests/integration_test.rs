//! Integration tests for SigmaOS-mint-update-notifier.

use sigmaos_mint_update_notifier_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
