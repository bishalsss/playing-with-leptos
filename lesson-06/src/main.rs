use leptos::prelude::*;
use leptos_meta::*;
use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Clone, Debug)]
struct Transaction {
    id: String,
    description: String,
    amount: f64,
    category: String,
    date: DateTime<Local>,
    transaction_type: TransactionType,
}

#[derive(Clone, Debug, PartialEq)]
enum TransactionType {
    Income,
    Expense,
}

impl TransactionType {
    fn as_str(&self) -> &'static str {
        match self {
            TransactionType::Income => "Income",
            TransactionType::Expense => "Expense",
        }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            description: String::new(),
            amount: 0.0,
            category: String::new(),
            date: Local::now(),
            transaction_type: TransactionType::Expense,
        }
    }
}

#[component]
fn ExpenseTracker() -> impl IntoView {
    // Initial transactions
    let initial_transactions = vec![
        Transaction {
            description: "Monthly Salary".to_string(),
            amount: 3500.00,
            category: "Salary".to_string(),
            transaction_type: TransactionType::Income,
            ..Default::default()
        },
        Transaction {
            description: "Groceries".to_string(),
            amount: 125.75,
            category: "Food".to_string(),
            ..Default::default()
        },
        Transaction {
            description: "Internet Bill".to_string(),
            amount: 65.00,
            category: "Utilities".to_string(),
            ..Default::default()
        },
        Transaction {
            description: "Coffee Shop".to_string(),
            amount: 4.50,
            category: "Food".to_string(),
            ..Default::default()
        },
        Transaction {
            description: "Freelance Work".to_string(),
            amount: 800.00,
            category: "Freelance".to_string(),
            transaction_type: TransactionType::Income,
            ..Default::default()
        },
    ];

    // State - FIXED: Using signal() instead of create_signal()
    let (transactions, set_transactions) = signal(initial_transactions);
    let (form, set_form) = signal(Transaction::default());
    let (filter_category, set_filter_category) = signal(String::new());
    let (filter_type, set_filter_type) = signal::<Option<TransactionType>>(None);
    let (editing_id, set_editing_id) = signal::<Option<String>>(None);

    // Computed values
    let total_income = move || {
        transactions.get()
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Income)
            .map(|t| t.amount)
            .sum::<f64>()
    };

    let total_expenses = move || {
        transactions.get()
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Expense)
            .map(|t| t.amount)
            .sum::<f64>()
    };

    let balance_value = move || total_income() - total_expenses();
    let is_positive = move || balance_value() >= 0.0;
    let is_negative = move || balance_value() < 0.0;

    let categories = move || {
        let mut cats: Vec<String> = transactions.get()
            .iter()
            .map(|t| t.category.clone())
            .collect();
        cats.sort();
        cats.dedup();
        cats
    };

    let filtered_transactions = move || {
        transactions.get()
            .into_iter()
            .filter(|t| {
                let category_match = filter_category.get().is_empty() || 
                    t.category.to_lowercase().contains(&filter_category.get().to_lowercase());
                let type_match = filter_type.get().is_none() || 
                    Some(&t.transaction_type) == filter_type.get().as_ref();
                category_match && type_match
            })
            .collect::<Vec<_>>()
    };

    let category_totals = move || {
        let mut totals = std::collections::HashMap::new();
        for transaction in transactions.get().iter() {
            let entry = totals.entry(transaction.category.clone()).or_insert((0.0, 0.0));
            match transaction.transaction_type {
                TransactionType::Income => entry.0 += transaction.amount,
                TransactionType::Expense => entry.1 += transaction.amount,
            }
        }
        totals
    };

    // Actions
    let add_transaction = move |_| {
        if !form.get().description.trim().is_empty() && form.get().amount > 0.0 {
            set_transactions.update(|transactions| {
                let mut new_transaction = form.get();
                new_transaction.id = Uuid::new_v4().to_string();
                new_transaction.date = Local::now();
                transactions.push(new_transaction);
            });
            set_form.set(Transaction::default());
        }
    };

    let update_transaction = move |_| {
        if let Some(id) = editing_id.get() {
            set_transactions.update(|transactions| {
                if let Some(transaction) = transactions.iter_mut().find(|t| t.id == id) {
                    transaction.description = form.get().description.clone();
                    transaction.amount = form.get().amount;
                    transaction.category = form.get().category.clone();
                    transaction.transaction_type = form.get().transaction_type.clone();
                }
            });
            set_editing_id.set(None);
            set_form.set(Transaction::default());
        }
    };

    let delete_transaction = move |id: String| {
        set_transactions.update(|transactions| {
            transactions.retain(|t| t.id != id);
        });
    };

    let edit_transaction = move |transaction: Transaction| {
        set_form.set(transaction.clone());
        set_editing_id.set(Some(transaction.id));
    };

    let cancel_edit = move |_| {
        set_editing_id.set(None);
        set_form.set(Transaction::default());
    };

    let reset_filters = move |_| {
        set_filter_category.set(String::new());
        set_filter_type.set(None);
    };

    let clear_all = move |_| {
        set_transactions.set(vec![]);
    };

    view! {
        <div class="expense-tracker">
            <div class="header">
                <h1>"💰 Expense Tracker"</h1>
                <p>"Track your income and expenses in real-time"</p>
            </div>

            <div class="summary">
                <div class="summary-item">
                    <h3>"Total Income"</h3>
                    <p class="income">{format!("${:.2}", total_income())}</p>
                </div>
                <div class="summary-item">
                    <h3>"Total Expenses"</h3>
                    <p class="expense">{format!("${:.2}", total_expenses())}</p>
                </div>
                <div class="summary-item">
                    <h3>"Current Balance"</h3>
                    <p 
                        class="balance"
                        class:positive=is_positive
                        class:negative=is_negative
                    >
                        {format!("${:.2}", balance_value())}
                    </p>
                </div>
            </div>

            <div class="form-section">
                <div class="form-container">
                    <h2>{move || if editing_id.get().is_some() { "Edit Transaction" } else { "Add New Transaction" }}</h2>
                    
                    <div class="form-row">
                        <div class="form-group">
                            <label for="description">"Description"</label>
                            <input
                                id="description"
                                type="text"
                                placeholder="What was this transaction for?"
                                prop:value=move || form.get().description
                                on:input=move |ev| set_form.update(|f| f.description = event_target_value(&ev))
                            />
                        </div>
                        
                        <div class="form-group">
                            <label for="amount">"Amount ($)"</label>
                            <input
                                id="amount"
                                type="number"
                                step="0.01"
                                min="0"
                                placeholder="0.00"
                                prop:value=move || form.get().amount.to_string()
                                on:input=move |ev| {
                                    let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                    set_form.update(|f| f.amount = value);
                                }
                            />
                        </div>
                    </div>

                    <div class="form-row">
                        <div class="form-group">
                            <label for="category">"Category"</label>
                            <input
                                id="category"
                                type="text"
                                placeholder="e.g., Food, Salary, Utilities"
                                list="categories"
                                prop:value=move || form.get().category
                                on:input=move |ev| set_form.update(|f| f.category = event_target_value(&ev))
                            />
                            <datalist id="categories">
                                {move || categories().into_iter()
                                    .map(|cat| view! { <option value=cat /> })
                                    .collect_view()}
                            </datalist>
                        </div>
                        
                        <div class="form-group">
                            <label>"Type"</label>
                            <div class="radio-group">
                                <label class="radio-option">
                                    <input
                                        type="radio"
                                        name="type"
                                        checked=move || form.get().transaction_type == TransactionType::Income
                                        on:change=move |_| set_form.update(|f| f.transaction_type = TransactionType::Income)
                                    />
                                    <span>"Income"</span>
                                </label>
                                <label class="radio-option">
                                    <input
                                        type="radio"
                                        name="type"
                                        checked=move || form.get().transaction_type == TransactionType::Expense
                                        on:change=move |_| set_form.update(|f| f.transaction_type = TransactionType::Expense)
                                    />
                                    <span>"Expense"</span>
                                </label>
                            </div>
                        </div>
                    </div>

                    <div class="form-actions">
                        <Show when=move || editing_id.get().is_some()
                            fallback=move || view! {
                                <button class="submit-btn" on:click=add_transaction>
                                    "Add Transaction"
                                </button>
                            }>
                            <div style="display: flex; gap: 10px;">
                                <button class="submit-btn" on:click=update_transaction>
                                    "Update Transaction"
                                </button>
                                <button class="reset-btn" on:click=cancel_edit>
                                    "Cancel"
                                </button>
                            </div>
                        </Show>
                    </div>
                </div>
            </div>

            <div class="filter-section">
                <div class="filter-controls">
                    <input
                        type="text"
                        placeholder="Filter by category..."
                        prop:value=filter_category
                        on:input=move |ev| set_filter_category.set(event_target_value(&ev))
                    />
                    
                    <select on:change=move |ev| {
                        let value = event_target_value(&ev);
                        set_filter_type.set(match value.as_str() {
                            "income" => Some(TransactionType::Income),
                            "expense" => Some(TransactionType::Expense),
                            _ => None,
                        });
                    }>
                        <option value="">"All Types"</option>
                        <option value="income">"Income Only"</option>
                        <option value="expense">"Expense Only"</option>
                    </select>
                    
                    <button class="reset-btn" on:click=reset_filters>
                        "Clear Filters"
                    </button>
                </div>
            </div>

            <div class="transactions-section">
                <h2>"Recent Transactions"</h2>
                
                <Show when=move || !filtered_transactions().is_empty()
                    fallback=move || view! {
                        <div class="empty-state">
                            <h3>"No transactions found"</h3>
                            <p>"Add your first transaction using the form above!"</p>
                        </div>
                    }>
                    <div class="transactions-list">
                        <For
                            each=filtered_transactions
                            key=|transaction| transaction.id.clone()
                            children=move |transaction| {
                                let tx_for_edit = transaction.clone();
                                let tx_for_delete = transaction.clone();
                                let is_income = transaction.transaction_type == TransactionType::Income;
                                view! {
                                    <div class="transaction"
                                         class:income=is_income
                                         class:expense=!is_income>
                                        <div class="transaction-info">
                                            <h3>{transaction.description}</h3>
                                            <div class="transaction-meta">
                                                <span class="category">{transaction.category}</span>
                                                <span class="date">{transaction.date.format("%b %d, %Y").to_string()}</span>
                                                <span class="type">{transaction.transaction_type.as_str()}</span>
                                            </div>
                                        </div>
                                        <div class="transaction-amount"
                                             class:positive=is_income
                                             class:negative=!is_income>
                                            {if is_income {
                                                "+"
                                            } else {
                                                "-"
                                            }}
                                            {format!("${:.2}", transaction.amount)}
                                        </div>
                                        <div class="transaction-actions">
                                            <button class="edit-btn" on:click=move |_| edit_transaction(tx_for_edit.clone())>
                                                "Edit"
                                            </button>
                                            <button class="delete-btn" on:click=move |_| delete_transaction(tx_for_delete.id.clone())>
                                                "Delete"
                                            </button>
                                        </div>
                                    </div>
                                }
                            }
                        />
                    </div>
                    
                    <div style="margin-top: 20px; text-align: center;">
                        <button class="reset-btn" on:click=clear_all>
                            "Clear All Transactions"
                        </button>
                    </div>
                </Show>
            </div>

            <div class="chart-section">
                <div class="chart-container">
                    <h3>"Spending by Category"</h3>
                    <div class="chart-bars">
                        <For
                            each=move || {
                                let totals = category_totals();
                                let max = totals.values()
                                    .map(|(income, expense)| income + expense)
                                    .fold(0.0, f64::max);
                                
                                let mut items: Vec<_> = totals.into_iter()
                                    .map(move |(category, (income, expense))| (category, income + expense, max))
                                    .collect();
                                items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                                items
                            }
                            key=|(category, _, _)| category.clone()
                            children=move |(category, total, max)| {
                                let height = if max > 0.0 { (total / max) * 180.0 } else { 0.0 };
                                view! {
                                    <div class="chart-bar" style=format!("height: {}px", height)>
                                        <div class="chart-bar-label">
                                            <div>{category}</div>
                                            <div style="font-size: 0.8rem; color: #94a3b8;">
                                                {format!("${:.0}", total)}
                                            </div>
                                        </div>
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Expense Tracker"/>
        <ExpenseTracker/>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
