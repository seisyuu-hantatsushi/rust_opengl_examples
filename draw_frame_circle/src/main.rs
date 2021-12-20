
mod draw_frame_circle;

use glfw::{Action, Context, Key};
use draw_frame_circle::*;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(800, 600, "Draw Frame Circle", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    let glrender = create_glrender(|s| window.get_proc_address(s) as *const _);

    while !window.should_close() {
	glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

	// rendering
	let (width,height) = window.get_size();
	glrender.render(width,height);
	window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },
        _ => {}
    }
}
