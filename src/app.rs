use yew::prelude::*;
use crate::navbar::Navbar;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Navbar />
            <a class="font-bold">{"Hello, world!"}</a>
        </main>
    }
}
