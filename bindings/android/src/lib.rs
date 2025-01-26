uniffi::setup_scaffolding!();

#[uniffi::export]
fn get_remaining_stack() -> Option<u64> {
    corelib::get_remaining_stack().map(|val| val as u64)
}

#[uniffi::export]
fn authorize(principal: &str, action: &str, resource: &str) -> String {
    corelib::authorize(principal, action, resource)
}
