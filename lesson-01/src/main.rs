use leptos::prelude::*;
use leptos_meta::*;

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Title text="My First Page"/>
        <h1>"Welcome to My Website"</h1>
        <p>"This is my first web page with Rust!"</p>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
