use leptos::prelude::*;
use leptos_meta::*;

#[component]
fn Calculator() -> impl IntoView {
    let (display, set_display) = create_signal("0".to_string());
    let (first_number, set_first_number) = create_signal(None::<f64>);
    let (operator, set_operator) = create_signal(None::<String>);
    let (waiting_for_second, set_waiting_for_second) = create_signal(false);

    let input_digit = move |digit: &str| {
        if waiting_for_second.get() {
            set_display.set(digit.to_string());
            set_waiting_for_second.set(false);
        } else {
            if display.get() == "0" {
                set_display.set(digit.to_string());
            } else {
                set_display.update(|d| d.push_str(digit));
            }
        }
    };

    let input_decimal = move |_| {
        if waiting_for_second.get() {
            set_display.set("0.".to_string());
            set_waiting_for_second.set(false);
        } else if !display.get().contains('.') {
            set_display.update(|d| d.push('.'));
        }
    };

    let clear_display = move |_| {
        set_display.set("0".to_string());
        set_first_number.set(None);
        set_operator.set(None);
        set_waiting_for_second.set(false);
    };

    let perform_operation = move |next_operator: &str| {
        let input_value = display.get().parse::<f64>().unwrap_or(0.0);
        
        if let Some(first) = first_number.get() {
            if let Some(op) = operator.get() {
                let result = match op.as_str() {
                    "+" => first + input_value,
                    "-" => first - input_value,
                    "×" => first * input_value,
                    "÷" => {
                        if input_value == 0.0 {
                            return;
                        }
                        first / input_value
                    }
                    _ => input_value,
                };
                
                set_display.set(result.to_string());
                set_first_number.set(Some(result));
            }
        } else {
            set_first_number.set(Some(input_value));
        }
        
        set_waiting_for_second.set(true);
        set_operator.set(Some(next_operator.to_string()));
    };

    let calculate_result = move |_| {
        let input_value = display.get().parse::<f64>().unwrap_or(0.0);
        
        if let Some(first) = first_number.get() {
            if let Some(op) = operator.get() {
                let result = match op.as_str() {
                    "+" => first + input_value,
                    "-" => first - input_value,
                    "×" => first * input_value,
                    "÷" => {
                        if input_value == 0.0 {
                            set_display.set("Error".to_string());
                            set_first_number.set(None);
                            set_operator.set(None);
                            return;
                        }
                        first / input_value
                    }
                    _ => input_value,
                };
                
                set_display.set(result.to_string());
                set_first_number.set(None);
                set_operator.set(None);
                set_waiting_for_second.set(false);
            }
        }
    };

    view! {
        <div class="calculator-container">
            <div class="header">
                <h1>"Calculator"</h1>
                <p>"A simple working calculator"</p>
            </div>

            <div class="display">
                <div class="result">{display}</div>
            </div>

            <div class="calculator-body">
                <button class="btn function" on:click=clear_display>
                    "C"
                </button>
                <button class="btn function" on:click=move |_| set_display.set("0".to_string())>
                    "CE"
                </button>
                <button class="btn function" on:click=move |_| {
                    let current = display.get();
                    if current.len() > 1 {
                        set_display.set(current[..current.len()-1].to_string());
                    } else {
                        set_display.set("0".to_string());
                    }
                }>
                    "⌫"
                </button>
                <button class="btn operator" on:click=move |_| perform_operation("÷")>
                    "÷"
                </button>
                
                <button class="btn number" on:click=move |_| input_digit("7")>
                    "7"
                </button>
                <button class="btn number" on:click=move |_| input_digit("8")>
                    "8"
                </button>
                <button class="btn number" on:click=move |_| input_digit("9")>
                    "9"
                </button>
                <button class="btn operator" on:click=move |_| perform_operation("×")>
                    "×"
                </button>
                
                <button class="btn number" on:click=move |_| input_digit("4")>
                    "4"
                </button>
                <button class="btn number" on:click=move |_| input_digit("5")>
                    "5"
                </button>
                <button class="btn number" on:click=move |_| input_digit("6")>
                    "6"
                </button>
                <button class="btn operator" on:click=move |_| perform_operation("-")>
                    "-"
                </button>
                
                <button class="btn number" on:click=move |_| input_digit("1")>
                    "1"
                </button>
                <button class="btn number" on:click=move |_| input_digit("2")>
                    "2"
                </button>
                <button class="btn number" on:click=move |_| input_digit("3")>
                    "3"
                </button>
                <button class="btn operator" on:click=move |_| perform_operation("+")>
                    "+"
                </button>
                
                <button class="btn number zero" on:click=move |_| input_digit("0")>
                    "0"
                </button>
                <button class="btn number" on:click=input_decimal>
                    "."
                </button>
                <button class="btn equals" on:click=calculate_result>
                    "="
                </button>
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Title text="Calculator"/>
        <Calculator/>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
