use std::ops::Not;
use leptos::*;

use log::{info, error};
use shared::calc::{Calc, Format};

#[derive(Clone)]
enum Message {
    Input(String),
    Output(String),
}

#[derive(Clone)]
struct State {
    calc: Calc,
    error: Option<String>,
    history: Vec<Message>,
    postfix: bool
}

#[component]
fn MessageItem(message: Message) -> impl IntoView {
    match message {
        Message::Output(value) => view! {
            <div class="d-flex flex-row justify-content-start mb-4">
                <div class="p-3 ms-3 rounded-pill bg-primary-subtle text-light-emphasis">
                    <p class="small mb-0">{ value }</p>
                </div>
            </div>
        }.into_view(),
        Message::Input(value) => view! {
            <div class="d-flex flex-row justify-content-end mb-4">
                <div class="p-3 me-3 border rounded-pill bg-light text-light-emphasis">
                    <p class="small mb-0">{ value }</p>
                </div>
            </div>
        }.into_view()
    }


}

#[component]
pub fn CalculatorComponent() -> impl IntoView {
    let (state, state_writer) = create_signal(State {
        calc: Calc::default(),
        error: None,
        history: Vec::new(),
        postfix: true,
    });

    let input_element_ref: NodeRef<html::Input> = create_node_ref();

    create_effect(move |_| {
        input_element_ref.get_untracked().and_then(|el| {
            el.autofocus();
            el.focus().ok()
        });
    });

    let on_submit = move |e: ev::SubmitEvent| {
        e.prevent_default();
        let input_element = input_element_ref.get().expect("input value is missing");
        let value = input_element.value();

        if value.is_empty() { return };

        state_writer.update(|state| {
            let mut next = state.calc.clone();
            let format = if state.postfix {
                info!("Using postfix");
                Format::Postfix(value.as_str())
            } else {
                info!("Using infix");
                Format::Infix(value.as_str())
            };
            match next.input(format) {
                Ok(_) => {
                    let eval = next
                        .eval()
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");

                    state.history.push(Message::Input(next.to_string()));
                    state.history.push(Message::Output(eval));
                    state.error = None;
                    state.calc = next;
                },
                Err(e) => {
                    error!("{}", e);
                    state.error = Some(e)
                }
            }
        });
        // Reset
        input_element.set_value("");
        input_element.focus().ok();
    };

    let on_click = move |e: ev::MouseEvent| {
        state_writer.update(|state| state.postfix = state.postfix.not());
    };

    let format = move || if state.with(|s| s.postfix) {
        "Postfix"
    } else {
        "Infix"
    };

    let is_invalid = move || state.with(|s| s.error.is_some());

    view! {
        <div id="calculator" class="row d-flex justify-content-center">
            <div class="col-md-8 col-lg-6 col-xl-4">
                <div class="card rounded shadow">
                    <header class="card-header d-flex justify-content-between align-items-center p-3 bg-black text-white rounded-top">
                        <p class="mb-0 fw-bold">Calculator</p>
                    </header>
                    <div class="card-body h-100">
                        <div class="overflow-y-auto">
                            {move || state.get().history.iter().map(|message| {
                                view! {
                                    <MessageItem message=message.clone() />
                                }
                            }).collect_view() }
                        </div>
                        <form on:submit=on_submit>
                            <div class="position-relative">
                                <input class="form-control" class:is-invalid=is_invalid
                                    node_ref=input_element_ref />
                                <Show when=is_invalid>
                                    <div class="invalid-feedback">{ move || state.with(|s| s.error.clone()) }</div>
                                </Show>
                                <div class="row form-text">
                                    <div class="col">(Press enter)</div>
                                    <div class="col text-end">
                                        <div class="form-check form-switch form-check-reverse">
                                            <label class="form-check-label">{ format }</label>
                                            <input class="form-check-input" type="checkbox" role="switch"
                                                checked=move || state.with(|s| s.postfix) on:click=on_click />
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
