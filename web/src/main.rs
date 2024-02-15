use yew::prelude::*;
use log::info;


#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onsubmit = {
        let counter = counter.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            let value = *counter + 1;
            counter.set(value);
            info!("All good!")
        }
    };

    html! {
        <>
            <header>
                <nav class="navbar bg-body-tertiary">
                    <div class="container-fluid">
                        <span class="navbar-brand mb-0 h1">{ "Calculator" }</span>
                    </div>
                </nav>
            </header>
            <main>
                <div class="container-xl px-4">
                    <form {onsubmit}>
                        <div class="mb-3">
                            <label for="exampleInputEmail1" class="form-label">{ "Counter" }</label>
                            <p>{ *counter }</p>
                            <input class="form-control" value={ "?" } />
                            <div class="form-text">{ "We'll never count with anyone else." } </div>
                        </div>
                        <button class="btn btn-primary">{ "+1" }</button>
                    </form>
                </div>
            </main>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::set_event_bubbling(false);
    yew::Renderer::<App>::new().render();
}