use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main>
            <h1>{ "Hello, World!" }</h1>
            <p>{ "Built with Rust and Yew running in WebAssembly." }</p>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
