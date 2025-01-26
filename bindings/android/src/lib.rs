uniffi::setup_scaffolding!();

#[uniffi::export]
fn get_remaining_stack() -> Option<u64> {
    coreapp::get_remaining_stack().map(|val| val as u64)
}

#[uniffi::export]
fn authorize(principal: &str, action: &str, resource: &str) -> String {
    coreapp::authorize(principal, action, resource)
}
