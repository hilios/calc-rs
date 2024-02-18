mod calculator;
mod counter;

use calculator::CalculatorComponent;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class="container-md">
            <header class="row  px-4">
                <div class="col gy-4">
                    <h1><i class="bi bi-calculator"></i> { "Calculator" }</h1>
                </div>
            </header>
            <main class="row px-4 justify-content-center">
                <div class="col gy-2">
                    <CalculatorComponent />
                </div>
            </main>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::set_event_bubbling(false);
    yew::Renderer::<App>::new().render();
}
