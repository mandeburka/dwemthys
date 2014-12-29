extern crate tcod;

use self::tcod::{Console, BackgroundFlag, KeyState};
use util::{Point, Bound};

pub trait RenderingComponent {
    fn new(Bound) -> Self;
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> KeyState;
}

pub struct TcodRenderingComponent {
    pub console: Console
}

impl RenderingComponent for TcodRenderingComponent {
    fn new(bounds: Bound) -> TcodRenderingComponent {
        let con = Console::init_root(
            (bounds.max.x + 1) as int,
            (bounds.max.y + 1) as int, 
            "libtcod Rust tutorial",
            false
        );
        TcodRenderingComponent { console: con }
    }
    
    fn before_render_new_frame(&mut self) {
        self.console.clear();
    }

    fn render_object(&mut self, position: Point, symbol: char) {
        self.console.put_char(position.x as int, position.y as int, symbol, BackgroundFlag::Set);
    }

    fn after_render_new_frame(&mut self) {
        Console::flush();
    }

    fn wait_for_keypress(&mut self) -> KeyState {
        Console::wait_for_keypress(true)
    }

}