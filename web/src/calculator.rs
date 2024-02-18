use std::rc::Rc;

use log::{error, info};
use yew::prelude::*;

use input_form::{InputData, InputForm};
use shared::calc::{Calc, Format};

mod input_form;

pub enum CalculatorAction {
    Input(InputData),
    Clear
}

#[derive(Clone)]
struct CalculatorState {
    calc: Calc,
    output: String,
    history: Vec<String>,
    error: Option<String>,
}

impl CalculatorState {
    fn input(&self, data: InputData) -> Self {
        let mut calc = self.calc.clone();
        let result = if data.infix {
            info!("Infix: {}", data.input);
            calc.input(Format::Infix(data.input.as_str()))
        } else {
            info!("Postfix: {}", data.input);
            calc.input(Format::Postfix(data.input.as_str()))
        };

        match result {
            Ok(_) => {
                let eval = calc
                    .eval()
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");

                let mut history = self.history.clone();
                history.insert(0, eval);

                let output = calc.to_string();

                Self {
                    calc,
                    output,
                    history,
                    error: None,
                }
            },
            Err(e) => {
                error!("Invalid input: {}", e);
                // Return the last valid state
                Self {
                    calc: self.calc.clone(),
                    output: self.output.clone(),
                    history: self.history.clone(),
                    error: Some(e),
                }
            },
        }
    }
}

impl Default for CalculatorState {
    fn default() -> Self {
        Self {
            calc: Calc::default(),
            output: String::from(""),
            history: Vec::new(),
            error: None,
        }
    }
}

impl Reducible for CalculatorState {
    type Action = CalculatorAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            CalculatorAction::Input(data) => self.input(data),
            CalculatorAction::Clear => CalculatorState::default()
        }.into()
    }
}

#[function_component]
pub fn CalculatorComponent() -> Html {
    let state = use_reducer(CalculatorState::default);

    let onsubmit = {
        let state = state.clone();
        Callback::from(move |data: InputData| {
            state.dispatch(CalculatorAction::Input(data));
        })
    };

    let clear = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(CalculatorAction::Clear))
    };

    html! {
        <>
            <InputForm {onsubmit} ~error={state.error.clone()} />
            <div class="row py-2">
                <div class="col">
                    <label for="history" class="form-label">{ "Output:" }</label>
                    <textarea class="form-control" rows="10" readonly=true value={state.history.clone().join("\n")}></textarea>
                </div>
            </div>
            <div class="row py-2">
                <div class="col">
                    <label for="formula" class="form-label">{ "Equation:" }</label>
                    <input id="input" class="form-control" readonly=true value={ state.output.clone() } />
                </div>
                <div class="col-1 align-self-end">
                    <button type="button" class="btn btn-secondary btn-sm" onclick={clear}>{ "Clear" }</button>
                </div>
            </div>
        </>
    }
}