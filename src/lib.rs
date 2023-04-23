mod random;
mod snake;

use std::{cell::RefCell, rc::Rc, sync::Arc};

use js_sys::Function;
use snake::Game;
use wasm_bindgen::prelude::*;
use web_sys::{console, window, HtmlElement, HtmlDivElement};

/*
   Frontend Part controlling the divs -> rendering those out etc.
*/

// this rust closure will get translated to a javascript closure so we can put it into the timeout below:
// because of this closure we have to move the game in there -> we have to make the game reference counted (above)
// then finally use RefCell and .borrow_mut() to get mutable access of the inner elements of game
// - also we need to make this persist this so we made it static:
thread_local! {
    static  GAME: Rc<RefCell<Game>> = Rc::new(RefCell::new(Game::new(15,10)));

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
    });

    render();
}

/// using websis we render the current game state
pub fn render() {
    let document = window().unwrap_throw().document().unwrap_throw();

    // get values out of game state:
    let height = GAME.with(|game| game.borrow().height);
    let width = GAME.with(|game| game.borrow().width);
    

    // get the root div element:
    let root = document
        .get_element_by_id("root")
        .unwrap_throw()
        .dyn_into::<HtmlElement>()
        .unwrap_throw();
    root.set_inner_html("");

    // set Css:
    root.style().set_property("display", "inline-grid").unwrap_throw();
    root.style().set_property("grid-template", &format!(
            "repeat({}, auto) / repeat({}, auto)", height, width,
        )).unwrap_throw();

    // loop over the field and create divs:
    for x in 1..(width+1){
        for y in 1..(height+1) {
            let point = (x,y);
            let el = document
                .create_element("div")
                .unwrap_throw()
                .dyn_into::<HtmlDivElement>()
                .unwrap_throw();

            // set content of the point:
            let typ = GAME.with(|game| game.borrow().get_typ(&point));
            el.set_inner_text(typ);

            root.append_child(&el).unwrap_throw();

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
