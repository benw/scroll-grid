use masonry::app::{EventLoop, EventLoopBuilder};
use masonry::dpi::LogicalSize;
use winit::error::EventLoopError;
use winit::window::Window;
use xilem::view::{grid, label, portal, sized_box, GridExt};
use xilem::{WidgetView, Xilem};

fn main() -> Result<(), EventLoopError> {
    run(EventLoop::with_user_event())
}

fn run(event_loop: EventLoopBuilder) -> Result<(), EventLoopError> {
    let data = Foo;
    let app = Xilem::new(data, app_logic);
    let min_window_size = LogicalSize::new(400., 60.);
    let window_size = LogicalSize::new(600., 60.);
    let window_attributes = Window::default_attributes()
        .with_title("Foo")
        .with_resizable(true)
        .with_min_inner_size(min_window_size)
        .with_inner_size(window_size);
    app.run_windowed_in(event_loop, window_attributes)?;
    Ok(())
}

struct Foo;

fn app_logic(_data: &mut Foo) -> impl WidgetView<Foo> {
    let len = 30;
    let cells: Vec<_> = (0..len).map(|i| sized_box(label(format!("{i}"))).expand().grid_pos(0, i)).collect();

    sized_box(portal(sized_box(grid(cells, 8, len as i32)).height(20000.)))
}
