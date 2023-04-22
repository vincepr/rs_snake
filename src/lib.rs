mod random;
mod snake;

use std::{rc::Rc, cell::RefCell};

use js_sys::Function;
use snake::Game;
use wasm_bindgen::{prelude::*};
use web_sys::{console,window};

/*
    Frontend Part controlling the divs -> rendering those out etc.
 */

// defining a main() function in wasm:
#[wasm_bindgen(start)]
pub fn main() {
    // // console log:
    console::log_1(&"Rust compiled Snake is running!".into());

    let mut game = Rc::new(RefCell::new(Game::new(30,20)));

    // this rust closure will get translated to a javascript closure so we can put it into the timeout below:
    // because of this closure we have to move the game in there -> we have to make the game reference counted (above)
    // then finally use RefCell and .borrow_mut() to get mutable access of the inner elements of game
    let tick_closure = Closure::wrap(Box::new(move || game.borrow_mut().tick()) as Box<dyn FnMut()> );

    // get the window dom element then set a timeout for the tick callback:
    window()
        .unwrap_throw()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(), 500
        )
        .unwrap_throw();

    //game.borrow_mut().tick();

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

    }
}