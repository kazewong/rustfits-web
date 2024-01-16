use yew::prelude::*;
use crate::navbar::Navbar;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Navbar />
        </main>
    }
}
