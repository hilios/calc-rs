use std::ops::Deref;
use std::rc::Rc;

use log::{error, info, warn};
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
}

impl CalculatorState {
    fn input(&self, data: InputData) -> Self {
        let mut calc = self.calc.clone();
        let result = if data.postfix {
            info!("Postfix: {}", data.input);
            calc.input(Format::Postfix(data.input.as_str()))
        } else {
            info!("Infix: {}", data.input);
            calc.input(Format::Infix(data.input.as_str()))
        };

        match result {
            Ok(_) => {
                let mut history = self.history.clone();
                if history.is_empty() {
                    history.push(calc.to_string());
                } else {
                    history.insert(0, calc.to_string());
                }

                let output = calc
                    .eval()
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");

                Self {
                    calc,
                    output,
                    history,
                }
            },
            Err(e) => {
                error!("Invalid input: {}", e);
                // Return the last valid state
                self.deref().clone()
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
        }
    }
}

impl Reducible for CalculatorState {
    type Action = CalculatorAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            CalculatorAction::Input(data) => {
                info!("Data {:?}", data);
                self.input(data)
            },
            CalculatorAction::Clear => {
                warn!("Clear!");
                CalculatorState::default()
            }
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
            <InputForm {onsubmit} />
            <br/>
            <div class="row">
                <div class="col">
                    <label for="formula" class="form-label">{ "Output" }</label>
                    <input id="input" class="form-control" readonly=true value={ state.output.clone() } />
                </div>
            </div>
            <div class="row">
                <div class="col">
                    <label for="history" class="form-label">{ "History" }</label>
                    <textarea class="form-control" rows="10" readonly=true value={state.history.clone().join("\n")}></textarea>
                </div>
            </div>
            <br/>
            <div class="row">
                <div class="d-grid gap-2 d-md-block">
                    <button type="button" class="btn btn-secondary btn-sm" onclick={clear}>{ "Clear" }</button>
                </div>
            </div>
        </>
    }
}