mod random;
mod snake;

use std::{cell::RefCell, rc::Rc};

use js_sys::Function;
use snake::Game;
use wasm_bindgen::prelude::*;
use web_sys::{console, window, HtmlDivElement, HtmlElement, KeyboardEvent};

/*
   Frontend Part controlling the divs -> rendering those out etc.
*/

// this rust closure will get translated to a javascript closure so we can put it into the timeout below:
// because of this closure we have to move the game in there -> we have to make the game reference counted (above)
// then finally use RefCell and .borrow_mut() to get mutable access of the inner elements of game
// - also we need to make this persist this so we made it static (because rust lifetimes...)
thread_local! {
    // static game struct
    static  GAME: Rc<RefCell<Game>> = Rc::new(RefCell::new(Game::new(15,10)));

    // static closure for our ticker
    static TICK_CLOSURE: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        || {
            GAME.with(|game| game.borrow_mut().tick());
            render();
        }
    }) as Box<dyn FnMut()> );

    // static closure for keydown events:
    static HANDLER_KEYDOWN: Closure<dyn FnMut(web_sys::KeyboardEvent)> = Closure::wrap(Box::new({
        |ev:KeyboardEvent| {
            let code = ev.code();
            let dir = match &*code{
                "ArrowUp" => Some(snake::Direction::Up),
                "ArrowDown" => Some(snake::Direction::Down),
                "ArrowRight" => Some(snake::Direction::Right),
                "ArrowLeft" => Some(snake::Direction::Left),
                _ => None,
            };
            if let Some(dir) = dir{
                GAME.with(|game| game.borrow_mut().direction_change(dir))
            }
        }
    }))

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
                200,
            )
            .unwrap_throw();
    });

    // event handler for user input:
    HANDLER_KEYDOWN.with(|handler| {
        window()
            .unwrap_throw()
            .add_event_listener_with_callback(
                "keydown",
                handler.as_ref().dyn_ref::<Function>().unwrap_throw(),
            )
            .unwrap_throw();
    });
    
    // calling the first render (before the first tick)
    render();
}


/// using websys we render the current game state (we just redraw everything every tick)
pub fn render() {
    let document = window().unwrap_throw().document().unwrap_throw();
    // make our static closure of game reachable with just game.
    GAME.with(|game|{
        // get values out of game state:
    let height = game.borrow().height;
    let width = game.borrow().width;

    // get the root div element:
    let root = document
        .get_element_by_id("root")
        .unwrap_throw()
        .dyn_into::<HtmlElement>()
        .unwrap_throw();
    root.set_inner_html("");

    // set Css:
    root.style()
        .set_property("display", "inline-grid")
        .unwrap_throw();
    root.style()
        .set_property(
            "grid-template",
            &format!("repeat({}, auto) / repeat({}, auto)", height, width,),
        )
        .unwrap_throw();

    // loop over the field and create divs:
    for y in 1..(height + 1) {
        for x in 1..(width + 1) {
            let point = (x, y);
            let el = document
                .create_element("div")
                .unwrap_throw()
                .dyn_into::<HtmlDivElement>()
                .unwrap_throw();

            // set content of the point:
            let typ = game.borrow().get_typ(&point);
            //let debug_info = typ.to_owned()+&format!("{:?}",point);
            el.set_inner_text(typ);
            el.set_class_name("pixel"); //so we can css it

            root.append_child(&el).unwrap_throw();
        }
    }

    // adjust the highscore:
    // get the root div element:
    let root = document
        .get_element_by_id("points")
        .unwrap_throw()
        .dyn_into::<HtmlElement>()
        .unwrap_throw();
    let score_msg = game.borrow().get_score();
    root.set_inner_html(&score_msg);

    });
}