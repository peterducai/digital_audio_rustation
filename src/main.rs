use audioengine::*;
use flo_draw::*;
use flo_canvas::*;

///
/// Displays a filled circle in a window
///
pub fn main() {
    println!("init...");
    version::print_full_version();
    filter::filter_out();

    with_2d_graphics(|| {
        // Create a window
        let canvas      = create_drawing_window("Circle");
        

        // Draw a circle
        canvas.draw(|gc| {
            // Set up the canvas
            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);

            // Draw a circle
            gc.new_path();
            gc.circle(500.0, 500.0, 250.0);

            gc.fill_color(Color::Rgba(0.3, 0.6, 0.8, 1.0));
            gc.fill();

            gc.line_width(6.0);
            gc.stroke_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.stroke();
        });

        // Draw a circle
        canvas.draw(|gc| {

            // Draw a circle
            gc.new_path();
            gc.circle(300.0, 330.0, 150.0);

            gc.fill_color(Color::Rgba(0.3, 0.6, 0.8, 1.0));
            gc.fill();

            gc.line_width(6.0);
            gc.stroke_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.stroke();
        });
    });
}