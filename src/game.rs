extern crate tcod;

use self::tcod::KeyState;
use util::{Bound, Point};
use actor::Actor;
use rendering::{RenderingComponent, TcodRenderingComponent};

static mut LAST_KEYPRESS: Option<KeyState> = None;
static mut CHAR_LOCATION: Point = Point { x: 40, y: 25 };

pub struct Game<'a> {
	pub exit: 					bool,
	pub window_bounds: 			Bound,
	pub rendering_component:	Box<RenderingComponent + 'a>
}

impl<'a> Game<'a> {
	pub fn new() -> Game<'a> {
		let bound = Bound {
			min: Point { x: 0, y: 0},
			max: Point { x: 79, y: 49}
		};
		let rc: Box<TcodRenderingComponent> = box RenderingComponent::new(bound);
		Game {
			exit: false,
			window_bounds: bound,
			rendering_component: rc
		}
	}
	
	pub fn render(&mut self, npcs: &Vec<Box<Actor>>, c: &Actor) {
		self.rendering_component.before_render_new_frame();
		for o in npcs.iter() {
			o.render(&mut *self.rendering_component);
		}
	    c.render(&mut *self.rendering_component);
		self.rendering_component.after_render_new_frame();
	}

	pub fn update(&self, npcs: &mut Vec<Box<Actor>>, c: &mut Actor) {
	    c.update();
	    Game::set_character_location(c.position);
	    for o in npcs.iter_mut() {
	        o.update();
	    }
	}

	pub fn wait_for_keypress(&mut self) -> KeyState {
		let ks = self.rendering_component.wait_for_keypress();
		Game::set_last_keypress(ks);
		ks
	}

	pub fn get_last_keypress() -> Option<KeyState> {
		unsafe { LAST_KEYPRESS }
	}

	pub fn set_last_keypress(ks: KeyState) {
		unsafe { LAST_KEYPRESS = Some(ks) }
	}

	pub fn get_character_location() -> Point {
		unsafe { CHAR_LOCATION }
	}

	pub fn set_character_location(point: Point) {
		unsafe { CHAR_LOCATION = point; }
	}
}