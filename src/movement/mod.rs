extern crate tcod;

use self::tcod::KeyCode;
use self::tcod::Key::Special;
use std::rand::{Rng, task_rng};
use util::{Point, Contains, Bound, XPointRelation, YPointRelation, PointEquality};
use game::Game;

pub trait MovementComponent {
	fn new(Bound) -> Self;
	fn update(&self, Point) -> Point;
}

pub struct RandomMovementComponent {
	window_bounds: Bound
}

impl Copy for RandomMovementComponent {}


impl MovementComponent for RandomMovementComponent {
	fn new(bound: Bound) -> RandomMovementComponent {
		RandomMovementComponent { window_bounds: bound }
	}

	fn update(&self, point: Point) -> Point {
		let mut offset = Point { x: point.x, y: point.y };
		let offset_x = task_rng().gen_range(0, 3i32) - 1;
        match self.window_bounds.contains(offset.offset_x(offset_x)) {
            Contains::DoesContain => offset = offset.offset_x(offset_x),
            Contains::DoesNotContain => { return point; }
        }

        let offset_y = task_rng().gen_range(0, 3i32) - 1;
        match self.window_bounds.contains(offset.offset_y(offset_y)) {
            Contains::DoesContain => offset = offset.offset_y(offset_y),
            Contains::DoesNotContain => { return point; }
        }
        offset
	}
}

pub struct TcodMovementComponent {
    window_bounds: Bound
}

impl Copy for TcodMovementComponent {}

impl MovementComponent for TcodMovementComponent {
	fn new(bound: Bound) -> TcodMovementComponent {
		TcodMovementComponent { window_bounds: bound }
	}

	fn update(&self, point: Point) -> Point {
		let mut offset = Point { x: point.x, y: point.y };
		offset = match Game::get_last_keypress() {
			Some(keypress) => {
				match keypress.key {
					Special(KeyCode::Up) => {
		    			offset.offset_y(-1)
		    		},
		    		Special(KeyCode::Down) => {
		    			offset.offset_y(1)
		    		},
		    		Special(KeyCode::Left) => {
		    			offset.offset_x(-1)
		    		},
		    		Special(KeyCode::Right) => {
		    			offset.offset_x(1)
		    		}
		    		_ => { offset }
				}
			},
			None => { offset }
		};
		
		match self.window_bounds.contains(offset) {
			Contains::DoesContain 		=> { offset },
    		Contains::DoesNotContain 	=> { point }
		}
	}
}

pub struct AggroMovementComponent {
    window_bounds: Bound
}

impl MovementComponent for AggroMovementComponent {
	fn new(bound: Bound) -> AggroMovementComponent {
		AggroMovementComponent { window_bounds: bound }
	}

	fn update(&self, point: Point) -> Point {
		let char_point = Game::get_character_location();
		let mut offset = Point { x: 0, y: 0 };
		
		match point.compare_x(&char_point) {
			XPointRelation::RightOfPoint	=>	offset = offset.offset_x(-1),
			XPointRelation::LeftOfPoint		=> offset = offset.offset_x(1),
			XPointRelation::OnPointX		=> {}
		}
		
		match point.compare_y(&char_point) {
			YPointRelation::BelowPoint		=> offset = offset.offset_y(-1),
			YPointRelation::AbovePoint		=> offset = offset.offset_y(1),
			YPointRelation::OnPointY		=> {}
		}

		match point.offset(&offset).compare(&char_point) {
			PointEquality::PointsEqual 		=> { return point },
			PointEquality::PointsNotEqual	=> {
				match self.window_bounds.contains(point.offset(&offset)) {
					Contains::DoesContain 		=> { return point.offset(&offset); }
					Contains::DoesNotContain	=> { return point; }
				}
			}
		}
	}
}