extern crate piston_window;
use piston_window::*;
use std::f64::consts::PI;

// General constants
const MAX_VALUE: f64 = 9000.0; // Maximum RPM value
const DANGER_THRESHOLD: i32 = 6000; // RPM threshold for danger
const GAUGE_RADIUS: f64 = 150.0; // Radius of the gauge
const VALUE_STEP: f64 = 50.0; // Incremental step for RPM increase
const DECAY_RATE: f64 = 20.0; // Rate at which RPM decreases
const GAUGE_CENTER: f64 = 200.0; // Center position of the gauge
pub struct Gauge {
    current_value: f64,    // Current RPM value
    max_value: f64,        // Maximum RPM value
    danger_threshold: i32, // RPM threshold for danger
    radius: f64,           // Radius of the gauge
    center: f64,           // Center position of the gauge
    value_step: f64,       // Step for RPM increment
    decay_rate: f64,       // Rate of RPM decay
}

impl Gauge {
    // Create a new instance of the Gauge
    pub fn new() -> Self {
        Gauge {
            current_value: 0.0,
            max_value: MAX_VALUE,
            danger_threshold: DANGER_THRESHOLD,
            radius: GAUGE_RADIUS,
            center: GAUGE_CENTER,
            value_step: VALUE_STEP,
            decay_rate: DECAY_RATE,
        }
    }

    // Method to update the RPM
    pub fn update_rpm(&mut self, key_up_pressed: bool, dt: f64) {
        if key_up_pressed && self.current_value < self.max_value {
            self.current_value += self.value_step * dt * 100.0; // Increase RPM
            if self.current_value > self.max_value {
                self.current_value = self.max_value; // Clamp to max value
            }
        } else if !key_up_pressed && self.current_value > 0.0 {
            self.current_value -= self.decay_rate * dt * 100.0; // Decrease RPM
            if self.current_value < 0.0 {
                self.current_value = 0.0; // Clamp to zero
            }
        }
    }

    // Method to draw the gauge
    pub fn draw(&self, context: Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        // Dibuja el gauge (aproximación de un arco con 100 líneas)
        // Draw the gauge (approximation of an arc with 100 lines)
        let start_angle: f64 = 3.0 * PI / 4.0; // Start angle of the gauge
        let end_angle: f64 = 9.0 * PI / 4.0; // End angle of the gauge
        let segments: i32 = 100; // Number of segments to draw
        let angle_step: f64 = (end_angle - start_angle) / segments as f64; // Angle increment for segments

        // Draw the gauge arc
        for i in 0..segments {
            let angle1: f64 = start_angle + i as f64 * angle_step;
            let angle2: f64 = start_angle + (i + 1) as f64 * angle_step;

            let (x1, y1) = (
                self.center + self.radius * angle1.cos(),
                self.center + self.radius * angle1.sin(),
            );
            let (x2, y2) = (
                self.center + self.radius * angle2.cos(),
                self.center + self.radius * angle2.sin(),
            );

            line(
                [0.0, 0.0, 0.0, 1.0], // Black color
                2.0,                  // Line thickness
                [x1, y1, x2, y2],
                context.transform,
                graphics,
            );
        }

        // Draw RPM ticks
        for rpm in (0..=self.max_value as i32).step_by(100) {
            let rpm_percentage: f64 = rpm as f64 / self.max_value; // RPM percentage of max value
            let angle: f64 = start_angle + rpm_percentage * (end_angle - start_angle);

            let outer_x: f64 = self.center + self.radius * angle.cos();
            let outer_y: f64 = self.center + self.radius * angle.sin();

            // Determine line size based on RPM value
            let line_length: f64 = if rpm % 1000 == 0 {
                20.0 // Long tick for multiples of 1000
            } else if rpm % 500 == 0 {
                10.0 // Medium tick for multiples of 500
            } else {
                5.0 // Short tick for multiples of 100
            };

            // Determine line thickness
            let line_radius: f64 = if rpm % 1000 == 0 { 2.0 } else { 1.0 };

            let inner_x: f64 = self.center + (self.radius - line_length) * angle.cos();
            let inner_y: f64 = self.center + (self.radius - line_length) * angle.sin();

            // Change color to red if RPM is above the danger threshold
            let line_color: [f32; 4] = if rpm >= self.danger_threshold {
                [1.0, 0.0, 0.0, 1.0] // Red color
            } else {
                [0.0, 0.0, 0.0, 1.0] // Black color
            };

            // Draw the tick line
            line(
                line_color,
                line_radius,
                [outer_x, outer_y, inner_x, inner_y],
                context.transform,
                graphics,
            );

            // Draw RPM values (only for multiples of 1000)
            if rpm % 1000 == 0 {
                let text_x: f64 = self.center + (self.radius - 30.0) * angle.cos();
                let text_y: f64 = self.center + (self.radius - 30.0) * angle.sin();
                let rpm_text: String = format!("{}", rpm / 1000);

                // Change text color if RPM exceeds danger zone
                let text_color: [f32; 4] = if rpm >= self.danger_threshold {
                    [1.0, 0.0, 0.0, 1.0] // Red color
                } else {
                    [0.0, 0.0, 0.0, 1.0] // Black color
                };

                let transform: [[f64; 3]; 2] = context.transform.trans(text_x - 5.0, text_y + 5.0);
                text::Text::new_color(text_color, 16)
                    .draw(&rpm_text, glyphs, &context.draw_state, transform, graphics)
                    .unwrap();
            }
        }

        // Calculate needle angle
        let rpm_percentage: f64 = self.current_value / self.max_value;
        let needle_angle: f64 = start_angle + rpm_percentage * (end_angle - start_angle);

        // Draw the needle
        let (needle_x, needle_y) = (
            self.center + self.radius * needle_angle.cos(),
            self.center + self.radius * needle_angle.sin(),
        );
        line(
            [1.0, 0.0, 0.0, 1.0],                           // Red color
            2.0,                                            // Thickness
            [self.center, self.center, needle_x, needle_y], // Needle coordinates
            context.transform,
            graphics,
        );

        // Draw the needle circle
        let needle_circle: f64 = 5.0;
        ellipse(
            [1.0, 0.0, 0.0, 1.0],
            [
                self.center - needle_circle,
                self.center - needle_circle,
                needle_circle * 2.0,
                needle_circle * 2.0,
            ],
            context.transform,
            graphics,
        );

        // Length of the extension (base of the needle)
        let extension_length: f64 = 20.0;

        // Calculate coordinates for the needle extension (in the opposite direction)
        let extension_x: f64 = self.center - extension_length * needle_angle.cos();
        let extension_y: f64 = self.center - extension_length * needle_angle.sin();

        // Draw the needle extension (in the opposite direction)
        line(
            [1.0, 0.0, 0.0, 1.0], // Color of the extension (can use the same color as the needle)
            3.0,                  // Thickness of the line
            [self.center, self.center, extension_x, extension_y], // Coordinates of the extension line
            context.transform,
            graphics,
        );

        // Draw the current RPM text
        let text: String = format!("RPM: {:.0}", self.current_value);
        let transform: [[f64; 3]; 2] = context.transform.trans(150.0, 350.0);
        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 18)
            .draw(&text, glyphs, &context.draw_state, transform, graphics)
            .unwrap();
    }
}
