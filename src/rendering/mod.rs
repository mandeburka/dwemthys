extern crate tcod;

use self::tcod::{Console, BackgroundFlag, KeyState, Color};
use util::{Point, Bound};


pub trait WindowComponent {
    fn get_bounds(&self)        -> Bound;
    fn get_bg_color(&self)      -> Color;
    fn get_console(&mut self)   -> &mut Console;

    fn clear (&mut self) {
        let color = self.get_bg_color();
        let mut console = self.get_console();
        console.set_default_background(color);
        console.clear();
    }

    fn print_message(&mut self, x: int, y: int, alignment: self::tcod::TextAlignment, text:&str) {
        let mut console = self.get_console();
        console.print_ex(x, y, BackgroundFlag::Set, alignment, text);
    }
}

pub struct TcodStatsWindowComponent {
    pub console: Console,
    pub backgrounf_color: Color,
    bounds: Bound
}

impl TcodStatsWindowComponent {
    pub fn new(bounds: Bound) -> TcodStatsWindowComponent {
        let height  = bounds.max.y - bounds.min.y + 1;
        let width   = bounds.max.x - bounds.min.x + 1;
        let console = Console::new(width as int, height as int);
        let red = Color { r: 255, g: 0, b: 0 };
        TcodStatsWindowComponent {
            console: console,
            backgrounf_color: red,
            bounds: bounds
        }
    }
}

impl WindowComponent for TcodStatsWindowComponent {

    fn get_bounds(&self) -> Bound { self.bounds }
    fn get_bg_color(&self) -> Color { self.backgrounf_color }
    fn get_console(&mut self) -> &mut Console { &mut self.console }
}

pub struct TcodInputWindowComponent{
    pub console: Console,
    pub backgrounf_color: Color,
    bounds: Bound
}

impl TcodInputWindowComponent {
    pub fn new(bounds: Bound) -> TcodInputWindowComponent {
        let height  = bounds.max.y - bounds.min.y + 1;
        let width   = bounds.max.x - bounds.min.x + 1;
        let console = Console::new(width as int, height as int);
        let red = Color { r: 0, g: 255, b: 0 };
        TcodInputWindowComponent {
            console: console,
            backgrounf_color: red,
            bounds: bounds
        }
    }
}

impl WindowComponent for TcodInputWindowComponent {
    fn get_bounds(&self) -> Bound { self.bounds }
    fn get_bg_color(&self) -> Color { self.backgrounf_color }
    fn get_console(&mut self) -> &mut Console { &mut self.console }
}

pub struct TcodMessagesWindowComponent{
    pub console: Console,
    pub backgrounf_color: Color,
    bounds: Bound
}

impl TcodMessagesWindowComponent {
    pub fn new(bounds: Bound) -> TcodMessagesWindowComponent {
        let height  = bounds.max.y - bounds.min.y + 1;
        let width   = bounds.max.x - bounds.min.x + 1;
        let console = Console::new(width as int, height as int);
        let red = Color { r: 0, g: 0, b: 255 };
        TcodMessagesWindowComponent {
            console: console,
            backgrounf_color: red,
            bounds: bounds
        }
    }
}

impl WindowComponent for TcodMessagesWindowComponent {
    fn get_bounds(&self) -> Bound { self.bounds }
    fn get_bg_color(&self) -> Color { self.backgrounf_color }
    fn get_console(&mut self) -> &mut Console { &mut self.console }
}

pub struct TcodMapWindowComponent{
    pub console: Console,
    pub backgrounf_color: Color,
    bounds: Bound
}

impl TcodMapWindowComponent {
    pub fn new(bounds: Bound) -> TcodMapWindowComponent {
        let height  = bounds.max.y - bounds.min.y + 1;
        let width   = bounds.max.x - bounds.min.x + 1;
        let console = Console::new(width as int, height as int);
        let red = Color { r: 0, g: 0, b: 0 };
        TcodMapWindowComponent {
            console: console,
            backgrounf_color: red,
            bounds: bounds
        }
    }
}

impl WindowComponent for TcodMapWindowComponent {
    fn get_bounds(&self) -> Bound { self.bounds }
    fn get_bg_color(&self) -> Color { self.backgrounf_color }
    fn get_console(&mut self) -> &mut Console { &mut self.console }
}

pub trait RenderingComponent {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> KeyState;
    fn attach_window(&mut self, window: &mut WindowComponent);
}

pub struct TcodRenderingComponent {
    pub console: Console
}

impl TcodRenderingComponent {
    pub fn new(bounds: Bound) -> TcodRenderingComponent {
        let con = Console::init_root(
            (bounds.max.x + 1) as int,
            (bounds.max.y + 1) as int, 
            "libtcod Rust tutorial",
            false
        );
        TcodRenderingComponent { console: con }
    }
}

impl RenderingComponent for TcodRenderingComponent {
    
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

    fn attach_window(&mut self, window: &mut WindowComponent) {
        window.clear();
        window.print_message(0, 0, self::tcod::TextAlignment::Left, "Sup foo!");
        window.print_message(0, 1, self::tcod::TextAlignment::Left, "Nothin fool!");
        let bounds = window.get_bounds();
        let console = window.get_console();

        Console::blit(
            console, 0, 0, bounds.max.x as int + 1, bounds.max.y as int + 1, &mut self.console, 
            bounds.min.x as int, bounds.min.y as int, 1f32, 1f32
        );
    }

}
