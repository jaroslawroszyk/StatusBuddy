mod mocks;

use std::sync::atomic::Ordering;

use mocks::mock_mouse_controller::MockMouseController;
use auto_clicker_rs::autoclicker::AutoClicker;

#[test]
fn test_do_one_click_calls_mouse_methods() {
    let mouse = MockMouseController::new();
    let mut auto_clicker = AutoClicker::new(mouse.clone(), 1);
    auto_clicker.do_one_click(10, 10).unwrap();
    assert_eq!(mouse.move_calls.load(Ordering::SeqCst), 1);
    assert_eq!(mouse.click_calls.load(Ordering::SeqCst), 1);
}

#[test]
fn test_do_one_click_and_run_click_loop() {
    let mouse = MockMouseController::new();
    let mut auto_clicker = AutoClicker::new(mouse.clone(), 1);

    let (x, y) = (50, 60);

    auto_clicker.do_one_click(x, y).unwrap();
    assert_eq!(mouse.move_calls.load(Ordering::SeqCst), 1);
    assert_eq!(mouse.click_calls.load(Ordering::SeqCst), 1);

    auto_clicker.get_running().store(true, Ordering::SeqCst);
    let mut auto_clicker_clone = auto_clicker;

    let handle = std::thread::spawn(move || {
        for _ in 0..3 {
            auto_clicker_clone.do_one_click(x, y).unwrap();
        }
        auto_clicker_clone
            .get_running()
            .store(false, Ordering::SeqCst);
    });

    handle.join().unwrap();

    assert!(mouse.move_calls.load(Ordering::SeqCst) >= 3);
    assert!(mouse.click_calls.load(Ordering::SeqCst) >= 3);
}
