extern crate sdl2;
extern crate dust;
extern crate image;

use self::image::{GenericImage};

mod scene_objects;

use std::process;

use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

use dust::*;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    #[cfg(target_os = "macos")] // Use OpenGL 4.1 since that is the newest version supported on macOS
    {
        let gl_attr = video_ctx.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);
    }

    let width: usize = 900;
    let height: usize = 700;
    let screen = screen::Screen {width, height};
    let window = video_ctx
        .window("Dust", width as u32, height as u32)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| video_ctx.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Renderer
    let renderer = pipeline::DeferredPipeline::create(&gl, &screen, false).unwrap();

    // Camera
    let mut camera = camera::PerspectiveCamera::new(vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),screen.aspect(), 0.1, 1000.0);

    let cube = mesh_generator::create_cube().unwrap();
    let mut textured_box = objects::ShadedMesh::create(&gl, &cube);
    let img = image::open("examples/assets/textures/test_texture.jpg").unwrap();
    let mut texture = texture::Texture2D::create(&gl).unwrap();
    texture.fill_with_u8(img.dimensions().0 as usize, img.dimensions().1 as usize, &img.raw_pixels());
    textured_box.texture = Some(texture);

    let back = image::open("examples/assets/textures/skybox_evening/back.jpg").unwrap();
    let front = image::open("examples/assets/textures/skybox_evening/front.jpg").unwrap();
    let top = image::open("examples/assets/textures/skybox_evening/top.jpg").unwrap();
    let left = image::open("examples/assets/textures/skybox_evening/left.jpg").unwrap();
    let right = image::open("examples/assets/textures/skybox_evening/right.jpg").unwrap();
    let mut texture3d = texture::Texture3D::create(&gl).unwrap();
    texture3d.fill_with(back.dimensions().0 as usize, back.dimensions().1 as usize,
                      [&right.raw_pixels(), &left.raw_pixels(), &top.raw_pixels(),
                          &top.raw_pixels(), &front.raw_pixels(), &back.raw_pixels()]);
    let skybox = objects::Skybox::create(&gl, texture3d);

    let light = dust::light::DirectionalLight::new(vec3(0.0, -1.0, 0.0));

    // set up event handling
    let mut events = ctx.event_pump().unwrap();

    // main loop
    let main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::MouseMotion {xrel, yrel, mousestate, .. } => {
                    if mousestate.left()
                    {
                        eventhandler::rotate(&mut camera, xrel, yrel);
                    }
                },
                Event::MouseWheel {y, .. } => {
                    eventhandler::zoom(&mut camera, y);
                },
                _ => {}
            }
        }

        // draw
        // Geometry pass
        renderer.geometry_pass_begin().unwrap();
        let transformation = Mat4::identity();
        textured_box.render(&transformation, &camera);
        skybox.render(&camera).unwrap();

        // Light pass
        renderer.light_pass_begin(&camera).unwrap();
        renderer.shine_directional_light(&light).unwrap();

        window.gl_swap_window();
    };

    renderer::set_main_loop(main_loop);
}
