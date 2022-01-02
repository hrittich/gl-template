mod gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::messagebox::{show_simple_message_box,MessageBoxFlag};
use std::time::Duration;

fn run() -> Result<(), String> {
    // Initialize SDL and the video subsystem
    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    // Create a window with given title and size.
    let window = video.window("{{project-name}}", 800, 600)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    // Once we have a window that is prepared for the use with OpenGL, we can load the addresses
    // of all OpenGL functions.
    gl::raw::load_with(|s| video.gl_get_proc_address(s) as *const _);

    // Create an OpenGL context for window, and make it current. Once you have an active OpenGL
    // context, you can start making OpenGL calls.
    let _gl_context = window.gl_create_context()?;

    unsafe { gl::raw::Viewport(0, 0, 800, 600) };

    let mut event_pump = sdl.event_pump()?;
    let mut i = 0;
    'main: loop {
        // Process events.
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                },
                _ => {}
            }
        }

        // Update state.
        i = (i + 1) % 256;
        let c = (i as f32 / 128.0 - 1.0).abs();

        // Draw
        unsafe {
            // Fill the back buffer with the given color.
            gl::raw::ClearColor(c, 0.3, 1.0 - c, 0.0);
            gl::raw::Clear(gl::raw::COLOR_BUFFER_BIT);
        }
        // Add further OpenGL commands here...

        // Show back buffer on screen.
        window.gl_swap_window();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        if let Err(_) = show_simple_message_box(MessageBoxFlag::ERROR, "Error", &e.to_string(), None) {
            // Cannot show the error as a message box. Hence, falling back to console output.
            eprintln!("Error: {}", e);
        }
    }
}
