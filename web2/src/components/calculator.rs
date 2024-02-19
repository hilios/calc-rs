use leptos::*;
use leptos::html::S;

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
pub fn MessageComponent(message: Message) -> impl IntoView {
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

    let on_submit = move |e: ev::SubmitEvent| {
        e.prevent_default();
        let input_element = input_element_ref.get().expect("input value is missing");
        let value = input_element.value();
        state_writer.update(|state| {
            let mut next = state.calc.clone();
            match next.input(Format::Postfix(value.as_str())) {
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
                    error!("Invalid input: {}", e);
                    state.error = Some(e)
                }
            }
        });
        // Reset
        input_element.set_value("");
        input_element.autofocus();
    };

    view! {
        <div id="calculator" class="row d-flex justify-content-center">
            <div class="col-md-8 col-lg-6 col-xl-4">
                <div class="card rounded shadow">
                    <div class="card-header d-flex justify-content-between align-items-center p-3 bg-black text-white rounded-top">
                        <p class="mb-0 fw-bold">Calculator</p>
                    </div>
                    <div class="card-body">
                        {move || state.get().history.iter().map(|message| {
                            view! {
                                <MessageComponent message=message.clone() />
                            }
                        }).collect_view() }

                        <form on:submit=on_submit>
                            <div class="mb-3">
                                <label class="form-label">Input</label>
                                <input class="form-control" class:is-invalid=move || { state.get().error.is_some() }
                                    node_ref=input_element_ref />
                                {move || if state.with(|s| s.error.is_some()) {
                                    view! {
                                        <div class="invalid-feedback">{ state.get().error.unwrap() }</div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="form-text">Use Postfix notation</div>
                                    }.into_any()
                                }}
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
