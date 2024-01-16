use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <button class="font-bold bg-red-500">
                {"Home"}
            </button>

        </nav>
    }
}