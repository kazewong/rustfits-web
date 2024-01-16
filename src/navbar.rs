use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <a class="navbar-brand" href="#">{ "Yew App" }</a>
            <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNavAltMarkup"
                aria-controls="navbarNavAltMarkup" aria-expanded="false" aria-label="Toggle navigation">
                <span class="navbar-toggler-icon" />
            </button>
            <div class="collapse navbar-collapse" id="navbarNavAltMarkup">
                <div class="navbar-nav">
                    <a class="nav-link active" href="#">{ "Home" }<span class="sr-only">{ "(current)" }</span></a>
                    <a class="nav-link" href="#">{ "Features" }</a>
                    <a class="nav-link" href="#">{ "Pricing" }</a>
                    <a class="nav-link disabled" href="#" tabindex="-1" aria-disabled="true">{ "Disabled" }</a>
                </div>
            </div>
        </nav>
    }
}