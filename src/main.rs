extern crate piston_window;
use piston_window::*;

mod gauge;
use gauge::Gauge;

fn main() {
    // Create a new Piston window for the RPM gauge
    let mut window: PistonWindow = WindowSettings::new("RPM Gauge", [400, 400])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Initialize the gauge and key press state
    let mut gauge: Gauge = Gauge::new();
    let mut key_up_pressed: bool = false;

    // Load the font for displaying text
    let ref font_path = "./assets/Roboto-Regular.ttf";
    let mut glyphs = window.load_font(font_path).unwrap();

    while let Some(event) = window.next() {
        // Detect when the up arrow key is pressed
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if key == Key::Up {
                key_up_pressed = true;
            }
        }

        // Detect when the up arrow key is released
        if let Some(Button::Keyboard(key)) = event.release_args() {
            if key == Key::Up {
                key_up_pressed = false;
            }
        }

        // Update the RPM of the gauge based on key press state
        if let Some(UpdateArgs { dt }) = event.update_args() {
            gauge.update_rpm(key_up_pressed, dt);
        }

        // Draw the gauge on the window
        window.draw_2d(&event, |context, graphics, device| {
            clear([1.0, 1.0, 1.0, 1.0], graphics); // Clear the background to white
            gauge.draw(context, graphics, &mut glyphs); // Draw the gauge

            glyphs.factory.encoder.flush(device); // Update the glyphs
        });
    }
}
