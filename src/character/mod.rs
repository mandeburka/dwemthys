extern crate tcod;

use util::{Point, Updates};
use rendering::RenderingComponent;
use movement::MovementComponent;

pub struct Character<'a> {
    pub position: 		Point,
    pub display_char: 	char,
    movement_component: Box<MovementComponent + 'a>
}

impl<'a> Character<'a> {
	pub fn new(x: i32, y: i32, dc: char, movement_component: Box<MovementComponent + 'a>) -> Character<'a> {
		Character {
            position: Point { x: x, y: y },
            display_char: dc,
            movement_component: movement_component
        }
	}
}

impl<'a> Updates for Character<'a> {
    fn update(&mut self) {
        self.position = self.movement_component.update(self.position);
    }

    fn render(&self, rendering_component: &mut RenderingComponent) {
        rendering_component.render_object(self.position, self.display_char)
    }
}
