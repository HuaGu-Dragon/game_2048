use game_2048::{Down, Enter, Game, Left, Right, Up};
use gpui::{
    App, AppContext, Application, Bounds, KeyBinding, WindowBounds, WindowOptions, px, size,
};

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("up", Up, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("down", Down, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("w", Up, None),
            KeyBinding::new("a", Left, None),
            KeyBinding::new("s", Down, None),
            KeyBinding::new("d", Right, None),
            KeyBinding::new("enter", Enter, None),
        ]);

        let bounds = Bounds::centered(None, size(px(500.), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(Game::new),
        )
        .unwrap();
    });
}
