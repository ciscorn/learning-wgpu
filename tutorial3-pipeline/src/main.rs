pub fn main() {
    env_logger::init();
    let (event_loop, window) = tutorial3::prepare_window();
    pollster::block_on(tutorial3::run(event_loop, window));
}
