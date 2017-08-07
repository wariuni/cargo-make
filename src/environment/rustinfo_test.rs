use super::*;

use log;

#[test]
fn load_with_values() {
    let logger = log::create("error");
    let rust_info = load(&logger);

    assert!(rust_info.version.is_some());
    assert!(rust_info.channel.is_some());
    assert!(rust_info.target_arch.is_some());
    assert!(rust_info.target_env.is_some());
    assert!(rust_info.target_os.is_some());
    assert!(rust_info.target_pointer_width.is_some());
    assert!(rust_info.target_vendor.is_some());
}
