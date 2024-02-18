slint::include_modules!();

// make a gui whit a button that increases a counter
pub fn example_gui() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 2);
        }
    });

    ui.run()
}