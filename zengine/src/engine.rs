/*
https://www.youtube.com/watch?v=6spBXIRsvto&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=1
https://www.youtube.com/watch?v=LuQpOBg_ebk&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=2
https://www.youtube.com/watch?v=MId3KcqcLic&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=3
https://www.youtube.com/watch?v=UtM7cZAlT3E&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=4
https://www.youtube.com/watch?v=q1lqQR6Ii5c&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=5
https://www.youtube.com/watch?v=ILQlXIN15Tw&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=6
https://www.youtube.com/watch?v=Vpt9461DiXQ&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=7
https://www.youtube.com/watch?v=27VrlPfHdqM&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=8
https://www.youtube.com/watch?v=_uGRDyX5MVA&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=9
https://www.youtube.com/watch?v=ZityqrnAR3s&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=10
https://www.youtube.com/watch?v=FqaNT7RkxL4&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=11
       12,13,14 Cambio del engine y teoria
https://www.youtube.com/watch?v=wlxBUDIzd3Y&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=15
https://www.youtube.com/watch?v=87IjPJtn13Y&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=16
https://www.youtube.com/watch?v=SD3CQvLGxxc&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=17
26:34
*/
use crate::core::{Store, Scene, Trans, System};

// https://docs.rs/gl/0.14.0/gl/#functions
#[derive(Default)]
pub struct Engine {
    store: Store,
    systems: Vec<Box<dyn System>>,
}

impl Engine {
    pub fn with_system<S: System>(mut self, system: S) -> Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn run<S: Scene>(mut self, mut scene: S) {
        println!("Inicia Engine");

        println!("Inicia Systems");
        for s in self.systems.iter_mut() {
            s.init(&mut self.store);
        }

        scene.on_start(&mut self.store);

        'main_loop: loop {
            for s in self.systems.iter_mut() {
                s.run(&mut self.store);
            }
            match scene.update(&mut self.store) {
                Trans::Quit => {
                    println!("Recibida transación Quit");
                    break 'main_loop;
                }
                _ => {}
            }
        }

        scene.on_stop(&mut self.store);

        println!("Systems dispose");
        for s in self.systems.iter_mut() {
            s.dispose(&mut self.store);
        }

        println!("Finaliza Engine");
    }
}

/*extern crate sdl2;
extern crate gl;

use sdl2::video::GLProfile;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::DisplayMode;
use sdl2::VideoSubsystem;
use sdl2::video::FullscreenType;

use crate::gl_utilities::shader::ShaderManager;
use crate::math::matrix4x4::Matrix4x4;
use crate::graphics::sprite::Sprite;
use crate::math::transform::Transform;
use crate::graphics::texture::Texture;
use crate::graphics::material::Material;
use crate::graphics::color::Color;


// LLamada de debugging
extern "system" fn dbg_callback(
    source: gl::types::GLenum,
    etype: gl::types::GLenum,
    _id: gl::types::GLuint,
    severity: gl::types::GLenum,
    _msg_length: gl::types::GLsizei,
    msg: *const gl::types::GLchar,
    _user_data: *mut std::ffi::c_void,
) {
    unsafe {
        println!(
            "dbg_callback {:#X} {:#X} {:#X} {:?}",
            source,
            etype,
            severity,
            std::ffi::CStr::from_ptr(msg),
        );
    }
}

pub struct EngineOption {
    pub title: String,
    pub fullscreen: bool,
    // Para adaptarse a diferentes resoluciones
    pub virtual_width: u32,
    pub virtual_height: u32,

    pub screen_width: u32,
    pub screen_height: u32,
}


pub fn start(option: EngineOption) {
    println!("¡Hola mundo soy el engine!");

    // Inicializamos ventana
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    // Diferentes configuraciones según sea windows o mac
    if cfg!(target_os="macos") {
        gl_attr.set_context_version(4, 1);
    } else {
        gl_attr.set_context_version(4, 3);
    }

    gl_attr.set_double_buffer(true);

    let mut window = video_subsystem
        .window(
            option.title.as_ref(),
            option.screen_width,
            option.screen_height,
        )
        .opengl()
        .build()
        .unwrap();

    if option.fullscreen {
        let display_mode = get_display_mode(&video_subsystem, &option);
        window.set_display_mode(display_mode).unwrap();
        window.set_fullscreen(FullscreenType::True).unwrap();
    }

    let _ctx = window.gl_create_context().unwrap();

    // Para cargar los punteros de las funciones openGL se usa load_with()
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);


    unsafe {
        // Esto no funciona bien en mac
        if cfg!(target_os = "macos") {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(Some(dbg_callback), std::ptr::null());
        }


        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    println!("Pixel format en el contexto de la ventana GL {:?}", window.window_pixel_format());
    println!("OpenGL Profile {:?} - OpenGL version {:?}", gl_attr.context_profile(), gl_attr.context_version());

    // Ojo, usamos ancho y alto virtual para  ajustarnos a la definición de la pantalla
    let projection = Matrix4x4::orthographics(
        0.0, option.virtual_width as f32, 0.0, option.virtual_height as f32, -100.0, 100.0);

    let mut shader_manager = ShaderManager::init();
    //let mut shader_manager2 = ShaderManager::init();

    let basic_shader = shader_manager.register(
        "basic",
        include_str!("basic.vert"),
        include_str!("basic.frag"),
    );

    // Cargamos texturas
    let texture1 = Texture::new("test.png");
    let texture2 = Texture::new("duck.png");


    let u_projection_location = basic_shader.get_uniform_location("u_projection");

    let mut sprite = Sprite::new(
        "test",
        basic_shader,
        Material::new(Color::white(), &texture2),
        None,
        None);
    sprite.load();

    // Creamos la matriz de transformación
    let mut transform = Transform::new();
    transform.position.x = 150.0;
    transform.position.y = 500.0;

    transform.rotation.z = 30.0;

    transform.scale.x = 50.0;
    transform.scale.y = 50.0;

    // Usar programa shader
    basic_shader.use_shader();

    // Arregla la pantalla según el aspect ratio
    resize(None, &option);

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Bucle principal ***********************************************************************
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'main_loop;
                }

                Event::KeyUp { keycode: Some(keycode), keymod, .. } => match (keycode, keymod) {
                    (Keycode::R, _) => {
                        println!("red");
                        unsafe {
                            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
                        }
                    }
                    (Keycode::G, _) => {
                        println!("green");
                        unsafe {
                            gl::ClearColor(0.0, 1.0, 0.0, 1.0);
                        }
                    }
                    (Keycode::B, _) => {
                        println!("blue");
                        unsafe {
                            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
                        }
                    }
                    _ => ()
                }
                _ => ()
            }
        }

        unsafe {
            gl::Disable(gl::SCISSOR_TEST);

            // En toda la pantalla
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Enable(gl::SCISSOR_TEST);

            // Ahora solamente el trozo de pantalla recortado
            gl::ClearColor(1.0, 1.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UniformMatrix4fv(
                u_projection_location,
                1,
                gl::FALSE,
                projection.data.as_ptr(),
            );
            // Pasamos la matriz de transformación
            sprite.draw(&transform.get_transformation_matrix());
        }
        window.gl_swap_window();
    }

    fn resize(new_size: Option<(i32, i32)>, option: &EngineOption) {
        let target_aspect_ratio = option.virtual_width as f32 / option.virtual_height as f32;

        let width: i32;
        let height: i32;

        match new_size {
            Some(new_size) => {
                width = new_size.0;
                height = new_size.1;
            }
            None => {
                width = option.screen_width as i32;
                height = option.screen_height as i32;
            }
        }
        let mut calculated_height = (width as f32 / target_aspect_ratio) as i32;
        let mut calculated_width = width;

        if calculated_height > height {
            calculated_height = height;
            calculated_width = (calculated_height as f32 * target_aspect_ratio) as i32;
        }

        let vp_x = (width / 2) - (calculated_width / 2);
        let vp_y = (height / 2) - (calculated_height / 2);

        unsafe {
            //gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Viewport(vp_x, vp_y, calculated_width, calculated_height);
            // Recortamos
            gl::Scissor(vp_x, vp_y, calculated_width, calculated_height);
            //gl::Enable(gl::SCISSOR_TEST);
        }
    }

    // Devuelve el modo de pantalla posible puesto en las opciones
    fn get_display_mode(video_subsystem: &VideoSubsystem, option: &EngineOption) -> DisplayMode {
        for i in 0..video_subsystem.num_display_modes(0).unwrap() {
            let display_mode = video_subsystem.display_mode(0, i).unwrap();
            if display_mode.w == option.screen_width as i32 && display_mode.h == option.screen_height as i32 {
                return display_mode;
            }
        }

        panic!("No hay DisplayMode para ancho {} y alto {}", option.screen_width, option.screen_height);
    }
}
*/
