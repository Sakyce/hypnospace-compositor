#![allow(unused)]

use smithay::{
    input::SeatState,
    wayland::{compositor::CompositorState, shm::ShmState, shell::xdg::XdgShellState, selection::data_device::DataDeviceState}, delegate_xdg_shell, delegate_compositor, delegate_shm, delegate_seat, delegate_data_device, backend::{renderer::{gles::GlesRenderer, Renderer, Frame, utils::draw_render_elements}, winit}, utils::Rectangle,
};
use wayland_server::{Display, ListeningSocket};

mod app;
mod clientstate;

use crate::app::App;

// Macros used to delegate protocol handling to types in the app state.
delegate_xdg_shell!(App);
delegate_compositor!(App);
delegate_shm!(App);
delegate_seat!(App);
delegate_data_device!(App);

fn main() {
    println!("Started");
    match run_winit() {
        Result::Ok(_) => println!("Finished!"),
        Result::Err(err) => panic!("{}", err)
    };
}

fn run_winit() -> Result<(), Box<dyn std::error::Error>> {
    let display: Display<App> = Display::new()?;
    let dh = display.handle();

    let compositor_state = CompositorState::new::<App>(&dh);
    let shm_state = ShmState::new::<App>(&dh, vec![]);
    let mut seat_state: SeatState<App> = SeatState::new();
    let seat: smithay::input::Seat<App> = seat_state.new_wl_seat(&dh, "winit");

    let mut state = {
        App {
            compositor_state,
            xdg_shell_state: XdgShellState::new::<App>(&dh),
            shm_state,
            seat_state,
            data_device_state: DataDeviceState::new::<App>(&dh),
            seat,
        }
    };

    let listener = ListeningSocket::bind("wayland-5")?;
    //let clients = Vec::new();

    let (mut backend, mut winit) = winit::init::<GlesRenderer>()?;

    let start_time = std::time::Instant::now();

    let keyboard = state.seat.add_keyboard(Default::default(), 200, 200);

    std::env::set_var("WAYLAND_DISPLAY", "wayland-5");

    // Create a weston terminal
    //std::process::Command::new("weston-terminal").spawn().ok();

    loop {

        // Do the rendering
        let screen_size = backend.window_size().physical_size;
        let full_rectangle = Rectangle::from_loc_and_size((0, 0), screen_size);


        let mut frame = backend
            .renderer()
            .render(screen_size, smithay::utils::Transform::Normal)?;
        frame.clear([1.0, 0.0, 0.0, 1.0], &[full_rectangle])?;
        frame.finish()?;

        //draw_render_elements(&mut frame, 1.0, &elements, )

        //backend.submit(Option::Some(&[full_rectangle]))?;
    }

    Ok(())
}
