mod controls;
// mod window;
use crate::core::Chip8;
// use crate::utils::clear_screen;

// use pixels::{Error, Pixels, SurfaceTexture};
// use winit::event::{Event, VirtualKeyCode};
// use winit::event_loop::{ControlFlow, EventLoop};
// use winit_input_helper::WinitInputHelper;

// pub fn render(mut chip8: Chip8) -> Result<(), Error> {
//     // Start event loop and create interface window
//     let mut input = WinitInputHelper::new();
//     let event_loop = EventLoop::new();
//     let (window, p_width, p_height, mut _scale_factor) = window::create("Chip 8", &event_loop);

//     // Prepare frame buffer
//     let surface_texture = SurfaceTexture::new(p_width, p_height, &window);
//     let mut pixels = Pixels::new(64, 32, surface_texture)?;

//     // Keyboard controls
//     let keyboard_controls = controls::get_keyboard_layout();

//     // winit_input_helper event loop
//     event_loop.run(move |event, _, control_flow| {
//         // When a draw is requested...
//         if let Event::RedrawRequested(_) = event {
//             let frame = pixels.get_frame();

//             // Write to frame buffer with data from Chip8 VRAM
//             for (screen_pixel, vram_pixel) in frame.chunks_exact_mut(4).zip(chip8.gfx.iter()) {
//                 if *vram_pixel == 0xFF {
//                     screen_pixel.iter_mut().for_each(|channel| *channel = 0xFF);
//                 } else {
//                     screen_pixel.iter_mut().for_each(|channel| *channel = 0x00);
//                 }
//             }

//             // Exit if framebuffer failed to render
//             if pixels
//                 .render()
//                 .map_err(|e| eprintln!("pixels.render() failed: {}", e))
//                 .is_err()
//             {
//                 *control_flow = ControlFlow::Exit;
//                 return;
//             }
//         }

//         // Input handler. I think this runs at the same Hz as the monitor's refresh rate
//         // which isn't great since it would tie the emu's clock speed to monitor's Vsync.
//         if input.update(&event) {
//             // Handle keyboard inputs
//             let held_keys: Vec<bool> = keyboard_controls
//                 .iter()
//                 .map(|key| input.key_held(*key))
//                 .collect();

//             // Handle esc key
//             if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
//                 *control_flow = ControlFlow::Exit;
//                 return;
//             }

//             // Adjust high DPI factor
//             if let Some(factor) = input.scale_factor_changed() {
//                 _scale_factor = factor;
//             }
//             // Resize the window
//             if let Some(size) = input.window_resized() {
//                 pixels.resize(size.width, size.height);
//             }

//             chip8.set_keys(held_keys);

//             // Emulate at 600 cycles/sec for a 60hz monitor
//             for _ in 0..10 {
//                 chip8.emulate_cycle();
//             }

//             // Log chip8 debug info in the terminal
//             clear_screen();
//             println!("{:?}", chip8);

//             // Redraw screen
//             window.request_redraw();
//         }
//     });
// }


// extern crate minifb;

use minifb::{Key, Window, WindowOptions, Scale};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub fn render(mut chip8: Chip8) {
    
    let opts = WindowOptions {
        scale: Scale::X16,
        ..WindowOptions::default()
    };
    
    let mut window = Window::new("Chip8", WIDTH, HEIGHT, opts)
    .unwrap_or_else(|err| {
        panic!("{}", err);
    });

    let mut framebuffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Keyboard controls
    let keyboard_controls = controls::get_keyboard_layout();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        let held_keys: Vec<bool> = keyboard_controls
        .iter()
        .map(|key| window.is_key_down(*key))
        .collect();

        chip8.set_keys(held_keys);
        chip8.emulate_cycle();

        for (framebuffer_pixel, vram_pixel) in framebuffer.iter_mut().zip(chip8.gfx.iter()) {
            if *vram_pixel == 0xFF {
                *framebuffer_pixel = 0xFFFFFFFF;
            } else {
                *framebuffer_pixel = 0x0000;
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&framebuffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
