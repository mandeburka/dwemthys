extern crate dwemthys;
extern crate tcod;

use tcod::{Console, KeyCode};
use tcod::Key::Special;

use dwemthys::game::Game;
use dwemthys::util::Updates;
use dwemthys::character::Character;
use dwemthys::npc::NPC;
use dwemthys::movement::{RandomMovementComponent, MovementComponent, TcodMovementComponent};



fn main() {
	let mut game = Game::new();
    
    let char_mc: Box<TcodMovementComponent> = box MovementComponent::new(game.window_bounds);
    let mut c = Character::new(40, 25, '@', char_mc);

    let cmc: Box<RandomMovementComponent> = box MovementComponent::new(game.window_bounds);
    let dmc: Box<RandomMovementComponent> = box MovementComponent::new(game.window_bounds);
    
    let mut npcs:Vec<Box<Updates>> = vec![
        box NPC::new(10, 10, 'd', dmc) as Box<Updates>,
        box NPC::new(40, 25, 'c', cmc) as Box<Updates>
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
