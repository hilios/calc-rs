use std::ops::{Deref, Not};

use log::error;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Default, Clone, Debug)]
pub struct InputData {
    pub input: String,
    pub postfix: bool,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<InputData>
}

#[function_component]
pub fn InputForm(props: &Props) -> Html {
    let state = use_state(|| InputData::default());

    let input_onchange = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut data = state.deref().clone();
                data.input = input.value();
                state.set(data);
            } else {
                error!("Couldn't find input element");
            }
        })
    };

    let format_onchange = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input = e.target().and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                let mut data = state.deref().clone();
                data.postfix = input.value() == "true";
                state.set(data);
            } else {
                error!("Couldn't find select element");
            }
        })
    };

    let onsubmit = {
        let state = state.clone();
        let form_onsubmit = props.onsubmit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let data = state.deref().clone();
            form_onsubmit.emit(data);

            let mut data = state.deref().clone();
            data.input = String::from("");
            state.set(data);
        })
    };

    html! {
        <form class="row" {onsubmit}>
            <div class="col-8">
                <div class="form-floating">
                    <input id="input" class="form-control" value={ state.input.clone() } onchange={input_onchange} />
                    <label for="input" class="form-label">{ "Input" }</label>
                </div>
            </div>
            <div class="col">
                <div class="form-floating">
                    <select class="form-select" id="format" onchange={format_onchange}>
                        <option value="true"  selected={ state.postfix }>{ "RPN" }</option>
                        <option value="false" selected={ state.postfix.not() }>{ "Natural" }</option>
                    </select>
                    <label for="format" class="form-label">{ "Format" }</label>
                </div>
            </div>
        </form>
    }
}