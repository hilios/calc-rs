use leptos::*;

mod components;

use components::calculator::CalculatorComponent;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let on_click = move |_| {
        set_count.update(|n| *n += 1);
    };

    view! {
        <button class="btn btn-primary" on:click=on_click>
            "Click me: "
            { count }
        </button>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| {
        view! {
            <div class="container py-5">
                <CalculatorComponent />
            </div>
        }
    })
}
