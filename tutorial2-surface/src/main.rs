pub fn main() {
    env_logger::init();
    let (event_loop, window) = tutorial2::prepare_window();
    pollster::block_on(tutorial2::run(event_loop, window));
}
