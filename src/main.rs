use audioengine::*;


///
/// Demonstrates how to follow the mouse cursor around by tracking events
///
/// `create_drawing_window_with_events` works like `create_drawing_window` except it will also return a stream of events
/// for that window. We can track these by monitoring the stream in a futures block created with `executor::block_on`.
///
/// In this case, we watch for pointer events and render a circle to track where the mouse is on a new layer. This
/// also demonstrates that layers can be used to partially update the canvas without erasing whatever else is drawn,
/// and shows that we can get canvas coordinates directly from a mouse event.
///
/// Is supposed to stop once the window is closed, but glutin appears to not always respond correctly to setting the
/// control flow to ControlFlow::Exit.
///
pub fn main() {

    println!("init...");
    version::print_full_version();
}
