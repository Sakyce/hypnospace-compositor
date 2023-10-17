#![allow(unused)]

use smithay::{
    input::SeatState,
    wayland::{compositor::CompositorState, shm::ShmState}, delegate_xdg_shell, delegate_compositor, delegate_shm, delegate_seat, delegate_data_device,
};
use wayland_server::Display;

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
    run_winit();
}

fn run_winit() {
    let display: Display<App> = Display::new().expect("Failed to create display");
    let dh = display.handle();

    let compositor_state = CompositorState::new::<App>(&dh);
    //let shm_state = ShmState::new::<App>(&dh, vec![]);
    //let mut seat_state = SeatState::new();
    //let seat = seat_state.new_wl_seat(&dh, "winit");

    loop {}
}
