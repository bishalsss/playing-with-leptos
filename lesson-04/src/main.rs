use leptos::prelude::*;
use leptos_meta::*;

#[derive(Clone)]
struct Todo {
    id: u32,
    text: String,
    completed: bool,
}

#[component]
fn TodoApp() -> impl IntoView {
    let (todos, set_todos) = create_signal(Vec::new());
    let (input_text, set_input_text) = create_signal(String::new());
    let (filter, set_filter) = create_signal("all".to_string());
    let next_id = RwSignal::new(1);

    let add_todo = move |_ev: leptos::ev::MouseEvent| {
        let text = input_text.get().trim().to_string();
        if !text.is_empty() {
            set_todos.update(|todos| {
                todos.push(Todo {
                    id: next_id.get(),
                    text: text.clone(),
                    completed: false,
                });
            });
            set_input_text.set(String::new());
            next_id.update(|id| *id += 1);
        }
    };

    let toggle_todo = move |id: u32| {
        set_todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = !todo.completed;
            }
        });
    };

    let delete_todo = move |id: u32| {
        set_todos.update(|todos| {
            todos.retain(|todo| todo.id != id);
        });
    };

    let filtered_todos = move || {
        todos.get().into_iter()
            .filter(|todo| match filter.get().as_str() {
                "active" => !todo.completed,
                "completed" => todo.completed,
                _ => true,
            })
            .collect::<Vec<_>>()
    };

    let total_todos = move || todos.get().len();
    let completed_todos = move || todos.get().iter().filter(|todo| todo.completed).count();
    let active_todos = move || total_todos() - completed_todos();

    let clear_completed = move |_| {
        set_todos.update(|todos| {
            todos.retain(|todo| !todo.completed);
        });
    };

    view! {
        <div class="app-container">
            <div class="header">
                <h1>"Todo List"</h1>
                <p>"Organize your tasks efficiently"</p>
            </div>

            <div class="add-todo">
                <input
                    class="todo-input"
                    type="text"
                    placeholder="What needs to be done?"
                    prop:value=input_text
                    on:input=move |ev| set_input_text.set(event_target_value(&ev))
                    on:keydown=move |ev| {
                        if ev.key() == "Enter" {
                            let text = input_text.get().trim().to_string();
                            if !text.is_empty() {
                                set_todos.update(|todos| {
                                    todos.push(Todo {
                                        id: next_id.get(),
                                        text: text.clone(),
                                        completed: false,
                                    });
                                });
                                set_input_text.set(String::new());
                                next_id.update(|id| *id += 1);
                            }
                        }
                    }
                />
                <button class="add-btn" on:click=add_todo>
                    "Add Task"
                </button>
            </div>

            <div class="filters">
                <button
                    class="filter-btn"
                    class:active=move || filter.get() == "all"
                    on:click=move |_| set_filter.set("all".to_string())
                >
                    "All"
                </button>
                <button
                    class="filter-btn"
                    class:active=move || filter.get() == "active"
                    on:click=move |_| set_filter.set("active".to_string())
                >
                    "Active"
                </button>
                <button
                    class="filter-btn"
                    class:active=move || filter.get() == "completed"
                    on:click=move |_| set_filter.set("completed".to_string())
                >
                    "Completed"
                </button>
            </div>

            <ul class="todo-list">
                <For
                    each=filtered_todos
                    key=|todo| todo.id
                    children=move |todo| {
                        view! {
                            <li class="todo-item" class:completed=todo.completed>
                                <input
                                    class="todo-checkbox"
                                    type="checkbox"
                                    checked=todo.completed
                                    on:change=move |_| toggle_todo(todo.id)
                                />
                                <span class="todo-text">{todo.text.clone()}</span>
                                <button class="delete-btn" on:click=move |_| delete_todo(todo.id)>
                                    "Delete"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>

            {move || if filtered_todos().is_empty() {
                Some(view! {
                    <div class="empty-state">
                        <div class="empty-text">
                            {match filter.get().as_str() {
                                "active" => "No active tasks!",
                                "completed" => "No completed tasks!",
                                _ => "No tasks yet. Add one above!",
                            }}
                        </div>
                    </div>
                })
            } else {
                None
            }}

            {move || if completed_todos() > 0 {
                Some(view! {
                    <div style="text-align: center; margin-top: 20px;">
                        <button
                            style="
                                background: #e74c3c;
                                color: white;
                                border: none;
                                padding: 12px 25px;
                                border-radius: 10px;
                                cursor: pointer;
                                font-size: 1rem;
                            "
                            on:click=clear_completed
                        >
                            "Clear Completed (" {completed_todos()} ")"
                        </button>
                    </div>
                })
            } else {
                None
            }}

            <div class="stats">
                <div class="stat">
                    <span class="stat-value">{total_todos}</span>
                    <span class="stat-label">"Total"</span>
                </div>
                <div class="stat">
                    <span class="stat-value">{active_todos}</span>
                    <span class="stat-label">"Active"</span>
                </div>
                <div class="stat">
                    <span class="stat-value">{completed_todos}</span>
                    <span class="stat-label">"Completed"</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Title text="Todo List"/>
        <TodoApp/>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
