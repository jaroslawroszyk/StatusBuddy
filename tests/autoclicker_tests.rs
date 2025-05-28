
mod mocks;

use std::sync::atomic::Ordering;

use mocks::mock_mouse_controller::MockMouseController;
use status_buddy::autoclicker::AutoClicker;


#[test]
fn test_autoclicker_creation() {
    let mouse = MockMouseController::new();
    let auto_clicker = AutoClicker::new(mouse, 2);
    assert_eq!(auto_clicker.get_click_interval_secs(), 2);
    assert_eq!(auto_clicker.get_running().load(Ordering::SeqCst), true);
}