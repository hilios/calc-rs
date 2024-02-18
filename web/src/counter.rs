use yew::prelude::*;
use log::info;

#[function_component]
pub fn CalculatorComponent() -> Html {
    let counter = use_state(|| 0);
    let onsubmit = {
        let counter = counter.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            info!("Add one!");
            counter.set(*counter + 1)
        })
    };

    html! {
        <form {onsubmit}>
            <div class="row">
                <div class="col">
                    <label for="exampleInputEmail1" class="form-label">{ "Counter" }</label>
                    <input class="form-control" value={ counter.clone().to_string() } />
                    <div class="form-text">{ "We'll never count with anyone else." } </div>
                </div>
            </div>
            <div class="row">
                <div class="col">
                    <button class="btn btn-primary">{ "+1" }</button>
                </div>
            </div>
        </form>
    }
}
