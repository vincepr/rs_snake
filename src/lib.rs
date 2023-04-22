mod random;
mod snake;

use std::{cell::RefCell, rc::Rc};

use js_sys::Function;
use snake::Game;
use wasm_bindgen::prelude::*;
use web_sys::{console, window, HtmlElement};

/*
   Frontend Part controlling the divs -> rendering those out etc.
*/

// this rust closure will get translated to a javascript closure so we can put it into the timeout below:
// because of this closure we have to move the game in there -> we have to make the game reference counted (above)
// then finally use RefCell and .borrow_mut() to get mutable access of the inner elements of game
// - also we need to make this persist this so we made it static:
thread_local! {
    static  GAME: Rc<RefCell<Game>> = Rc::new(RefCell::new(Game::new(30,20)));

    static TICK_CLOSURE: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        let game = GAME.with(|game| game.clone());
        move || game.borrow_mut().tick()
    }) as Box<dyn FnMut()> );

}

/// defining a main() function in wasm, entry point of the game
#[wasm_bindgen(start)]
pub fn main() {
    // // console log:
    console::log_1(&"Rust compiled Snake is running!".into());

    // first we unpack to get the closure (out of the static var)
    TICK_CLOSURE.with(|tick_closure| {
        // then we get the window dom element then set a timeout for the tick callback:
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                // and pass in the closure. That basicaly just contains GAME.tick().
                tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                500,
            )
            .unwrap_throw();
    })
}

/// using websis we render the current game state
pub fn render() {
    let root = window()
        .unwrap_throw()
        .document()
        .unwrap_throw()
        .get_element_by_id("root")
        .unwrap_throw()
        .dyn_into::<HtmlElement>()
        .unwrap_throw();
    root.set_inner_html("Hello World");

    root.style().set_property("display", "grid").unwrap_throw();
    root.style().set_property("grid-template", &format!(
            "repeat({}, auto) / repeat({}, auto)", 
            GAME.with(
                |game| game.height
            ), GAME.with(|game| game.width)
        ))
        .unwrap_throw();

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
