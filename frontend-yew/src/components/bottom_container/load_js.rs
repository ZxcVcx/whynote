use yew::prelude::*;

#[function_component(LoadJs)]
pub fn load_js() -> Html {
    html! {
        // load scripts
        <script src="js/bootstrap.bundle.min.js" integrity="sha384-YvpcrYf0tY3lHB60NNkmXc5s9fDVZLESaAA55NDzOxhy9GkcIdslK1eN7N6jIeHz"></script>
    }
}
