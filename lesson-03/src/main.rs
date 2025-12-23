use leptos::prelude::*;
use leptos_meta::*;

#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0i32);
    
    let message = move || {
        let c = count.get();
        if c == 0 {
            "Click the buttons to start counting!"
        } else if c > 0 && c < 10 {
            "Good start!"
        } else if c >= 10 && c < 50 {
            "Keep going!"
        } else if c >= 50 {
            "Wow, you're counting a lot!"
        } else {
            "Negative numbers! Interesting..."
        }
    };
    
    let is_positive = move || count.get() > 0;
    let is_negative = move || count.get() < 0;
    let is_even = move || count.get() % 2 == 0;
    
    view! {
        <div class="counter-container">
            <h1 class="title">"Interactive Counter"</h1>
            
            <div class="count-display">{count}</div>
            
            <div class="controls">
                <button class="btn btn-decrease" on:click=move |_| set_count.update(|n| *n -= 1)>
                    "Decrease"
                </button>
                <button class="btn btn-reset" on:click=move |_| set_count.set(0)>
                    "Reset"
                </button>
                <button class="btn btn-increase" on:click=move |_| set_count.update(|n| *n += 1)>
                    "Increase"
                </button>
            </div>
            
            <div class="message">{message}</div>
            
            <div class="stats">
                <div class="stat-row">
                    <span class="stat-label">"Is positive:"</span>
                    <span class="stat-value">{move || if is_positive() { "Yes" } else { "No" }}</span>
                </div>
                <div class="stat-row">
                    <span class="stat-label">"Is negative:"</span>
                    <span class="stat-value">{move || if is_negative() { "Yes" } else { "No" }}</span>
                </div>
                <div class="stat-row">
                    <span class="stat-label">"Is even:"</span>
                    <span class="stat-value">{move || if is_even() { "Yes" } else { "No" }}</span>
                </div>
                <div class="stat-row">
                    <span class="stat-label">"Absolute value:"</span>
                    <span class="stat-value">{move || count.get().abs()}</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Title text="Counter App"/>
        <Counter/>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
