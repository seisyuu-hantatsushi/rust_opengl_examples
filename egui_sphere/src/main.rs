
mod draw_sphere;

use std::time;

use glfw::{Context};
use egui_glfw_gl::egui::{vec2, Color32, Image, Pos2, Rect};

use draw_sphere::DrawSphere;

struct CameraParamInput {
    position: (f64, f64, f64),
    center: (f64, f64, f64),
    up: (f64, f64, f64),
}

struct AppControlContext {
    camera_parameter : CameraParamInput
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap_or_else(|e| panic!("failed to init GLFW. {}",e));

    let (mut window, events) = glfw.create_window(800, 600, "Draw Frame Sphere", glfw::WindowMode::Windowed)
        .unwrap_or_else(|| panic!("Failed to create GLFW window."));

    window.set_all_polling(true);
    window.set_mouse_button_polling(true);
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut painter = egui_glfw_gl::Painter::new(&mut window);
    let mut egui_ctx = egui::CtxRef::default();

    let (width, height) = window.get_framebuffer_size();
    let native_pixels_per_point = window.get_content_scale().0;
    let mut egui_input_state = egui_glfw_gl::EguiInputState::new(egui::RawInput {
        screen_rect: Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(width as f32, height as f32) / native_pixels_per_point,
        )),
        pixels_per_point: Some(native_pixels_per_point),
        ..Default::default()
    });
    let start_time = time::Instant::now();
    //let mut value_str: String = "0.0".to_owned();

    let mut control_context = AppControlContext {
	camera_parameter: CameraParamInput {
	    position: (0.0, 0.0, 0.0),
	    center: (0.0, 0.0, 0.0),
	    up: (0.0, 0.0, 0.0)
	}
    };

    let mut camera_position = (control_context.camera_parameter.position.0.to_string().to_owned(),
			       control_context.camera_parameter.position.1.to_string().to_owned(),
			       control_context.camera_parameter.position.2.to_string().to_owned());
    let mut camera_center   = (control_context.camera_parameter.center.0.to_string().to_owned(),
			       control_context.camera_parameter.center.1.to_string().to_owned(),
			       control_context.camera_parameter.center.2.to_string().to_owned());
    let mut camera_up       = (control_context.camera_parameter.up.0.to_string().to_owned(),
			       control_context.camera_parameter.up.1.to_string().to_owned(),
			       control_context.camera_parameter.up.2.to_string().to_owned());

    let draw_sphere = DrawSphere::create();

    while !window.should_close() {
	let mut camera_update = false;

	let (width,height) = window.get_size();
	let native_pixels_per_point = window.get_content_scale().0;

	egui_input_state.input.time = Some(start_time.elapsed().as_secs_f64());
	egui_ctx.begin_frame(egui_input_state.input.take());

	egui_input_state.input.screen_rect = Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(width as f32, height as f32) / native_pixels_per_point,
        ));
	egui_input_state.input.pixels_per_point = Some(native_pixels_per_point);

	egui::Window::new("Control").show(&egui_ctx, |ui| {
	    ui.set_max_width(100.0);
	    ui.group(|ui| {
		ui.label("camera");
		ui.horizontal(|ui| {
		    ui.set_max_width(20.0);
		    ui.label("position");
		    ui.label("x:");
		    ui.text_edit_singleline(&mut camera_position.0);
		    ui.label("y:");
		    ui.text_edit_singleline(&mut camera_position.1);
		    ui.label("z:");
		    ui.text_edit_singleline(&mut camera_position.2);
		});
		ui.horizontal(|ui| {
		    ui.set_max_width(20.0);
		    ui.label("center");
		    ui.label("x:");
		    ui.text_edit_singleline(&mut camera_center.0);
		    ui.label("y:");
		    ui.text_edit_singleline(&mut camera_center.1);
		    ui.label("z:");
		    ui.text_edit_singleline(&mut camera_center.2);
		});
		ui.horizontal(|ui| {
		    ui.set_max_width(20.0);
		    ui.label("up");
		    ui.label("x:");
		    ui.text_edit_singleline(&mut camera_up.0);
		    ui.label("y:");
		    ui.text_edit_singleline(&mut camera_up.1);
		    ui.label("z:");
		    ui.text_edit_singleline(&mut camera_up.2);
		});
		if ui.button("update").clicked() {
		    camera_update = true;
		}
	    });
	});
	let (egui_output, paint_cmds) = egui_ctx.end_frame();

	if camera_update {
	    let position = (camera_position.0.parse::<f64>(),
			    camera_position.1.parse::<f64>(),
			    camera_position.2.parse::<f64>());
	    let center = (camera_center.0.parse::<f64>(),
			  camera_center.1.parse::<f64>(),
			  camera_center.2.parse::<f64>());
	    let up = (camera_up.0.parse::<f64>(),
		      camera_up.1.parse::<f64>(),
		      camera_up.2.parse::<f64>());
	    match (position,center,up) {
		((Ok(px),Ok(py),Ok(pz)),(Ok(cx),Ok(cy),Ok(cz)),(Ok(ux),Ok(uy),Ok(uz))) => {
		    control_context.camera_parameter.position = (px,py,pz);
		    control_context.camera_parameter.center = (cx,cy,cz);
		    control_context.camera_parameter.up = (ux,uy,uz);
		},
		_ => {
		    println!("invalid input");
		    ()
		}
	    }
	}

	draw_sphere.render(width,height);

	//Handle cut, copy text from egui
        if !egui_output.copied_text.is_empty() {
            egui_glfw_gl::copy_to_clipboard(&mut egui_input_state, egui_output.copied_text);
        }

        let paint_jobs = egui_ctx.tessellate(paint_cmds);
        painter.paint_jobs(
            None,
            paint_jobs,
            &egui_ctx.texture(),
	    width,
	    height,
            native_pixels_per_point,
        );

	for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Close => window.set_should_close(true),
                _ => { egui_glfw_gl::handle_event(event, &mut egui_input_state); }
            }
        }

	window.swap_buffers();
        glfw.poll_events();
    }
}
