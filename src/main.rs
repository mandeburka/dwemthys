extern crate dwemthys;
extern crate tcod;

use tcod::{Console, KeyCode};
use tcod::Key::Special;

use dwemthys::game::Game;
use dwemthys::actor::Actor;
use dwemthys::rendering::WindowComponent;
use dwemthys::movement::MovementComponent;



fn main() {
	let mut game = Game::new();
    
    let mut c = Actor::heroine(game.map_window.get_bounds());
    
    let mut npcs:Vec<Box<Actor>> = vec![
        box Actor::dog(10, 10, game.map_window.get_bounds()),
        box Actor::cat(40, 25, game.map_window.get_bounds()),
        box Actor::kobold(20, 20, game.map_window.get_bounds()),
    ];
    
    //render
	game.render(&npcs, &c);
    while !(Console::window_closed() || game.exit) {
    	// wait for user input
    	let keypress = game.wait_for_keypress();
    	
    	// update game state
    	match keypress.key {
    		Special(KeyCode::Escape) 	=> game.exit = true,
    		_ 							=> {}
    	}

    	// update game state
    	game.update(&mut npcs, &mut c);

    	// render
    	game.render(&npcs, &c);
    }
}
