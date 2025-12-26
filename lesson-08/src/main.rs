use leptos::prelude::*;
use leptos_meta::*;

#[component]
fn MarkdownEditor() -> impl IntoView {
    // State for markdown content
    let (markdown, set_markdown) = create_signal(
        "# Welcome to Markdown Editor!\n\nType your markdown on the left and see the live preview on the right.\n\n## Features\n\n- **Real-time preview**\n- *Formatting tools*\n- `Code blocks`\n- Export options\n\n### Try it out!\n\n```rust\nfn main() {\n    println!(\"Hello, Markdown!\");\n}\n```\n\n> This is a blockquote\n\n[Learn more about Markdown](https://www.markdownguide.org/)".to_string()
    );
    
    // Character and line count
    let char_count = move || markdown.get().chars().count();
    let line_count = move || markdown.get().lines().count();
    
    // Formatting helper functions
    let insert_bold = move |_| {
        set_markdown.update(|md| md.push_str("**bold text**"));
    };
    
    let insert_italic = move |_| {
        set_markdown.update(|md| md.push_str("*italic text*"));
    };
    
    let insert_code = move |_| {
        set_markdown.update(|md| md.push_str("`inline code`"));
    };
    
    let insert_link = move |_| {
        set_markdown.update(|md| md.push_str("[link text](https://example.com)"));
    };
    
    let insert_heading = move |_| {
        set_markdown.update(|md| md.push_str("\n## New Heading"));
    };
    
    let insert_list = move |_| {
        set_markdown.update(|md| md.push_str("\n- List item"));
    };
    
    let insert_code_block = move |_| {
        set_markdown.update(|md| md.push_str("\n```\ncode here\n```"));
    };
    
    let insert_blockquote = move |_| {
        set_markdown.update(|md| md.push_str("\n> Blockquote"));
    };
    
    // Clear editor
    let clear_editor = move |_| {
        set_markdown.set("".to_string());
    };
    
    // Export functions - simplified
    let show_export = move |_| {
        let text = markdown.get();
        let _ = window().alert_with_message(&format!("Content has {} characters.\n\nFirst 100 chars:\n{}", text.len(), &text[0..text.len().min(100)]));
    };
    
    // Simple markdown to HTML conversion
    let html_preview = move || {
        let md = markdown.get();
        markdown_to_html(&md)
    };
    
    view! {
        <div class="editor-container">
            <div class="header">
                <h1>"Real-time Markdown Editor"</h1>
                <p>"Write markdown on the left, see live preview on the right"</p>
            </div>
            
            <div class="editor-wrapper">
                <div class="editor-section">
                    <div class="section-header">
                        <h2>"Editor"</h2>
                        <div class="stats">
                            <span>{move || format!("{} chars", char_count())}</span>
                            <span>{move || format!("{} lines", line_count())}</span>
                        </div>
                    </div>
                    
                    <textarea
                        class="editor-input"
                        prop:value=markdown
                        on:input=move |ev| {
                            set_markdown.set(event_target_value(&ev));
                        }
                        placeholder="Start typing your markdown here..."
                    />
                    
                    <div class="toolbar">
                        <button class="toolbar-btn" on:click=insert_bold title="Bold">
                            <i class="fas fa-bold"></i>
                        </button>
                        <button class="toolbar-btn" on:click=insert_italic title="Italic">
                            <i class="fas fa-italic"></i>
                        </button>
                        <button class="toolbar-btn" on:click=insert_code title="Inline Code">
                            <i class="fas fa-code"></i>
                        </button>
                        <button class="toolbar-btn" on:click=insert_link title="Link">
                            <i class="fas fa-link"></i>
                        </button>
                        <button class="toolbar-btn" on:click=insert_heading title="Heading">
                            <i class="fas fa-heading"></i>
                        </button>
                        <button class="toolbar-btn" on:click=insert_list title="List">
                            <i class="fas fa-list"></i>
                        </button>
                        <button class="toolbar-btn" on:click=insert_code_block title="Code Block">
                            <i class="fas fa-file-code"></i>
                        </button>
                        <button class="toolbar-btn" on:click=insert_blockquote title="Blockquote">
                            <i class="fas fa-quote-right"></i>
                        </button>
                        <button class="toolbar-btn secondary" on:click=clear_editor title="Clear">
                            <i class="fas fa-trash"></i>
                        </button>
                    </div>
                </div>
                
                <div class="preview-section">
                    <div class="section-header">
                        <h2>"Preview"</h2>
                        <div class="stats">
                            <span>"Live"</span>
                        </div>
                    </div>
                    
                    <div class="preview-content" inner_html=html_preview />
                </div>
            </div>
            
            <div class="export-buttons">
                <button class="toolbar-btn primary" on:click=show_export>
                    <i class="fas fa-info-circle"></i>
                    "Show Content Info"
                </button>
            </div>
            
            <div class="cheatsheet">
                <h3>"Markdown Cheatsheet"</h3>
                <div class="cheatsheet-grid">
                    <div class="cheatsheet-item">
                        <h4>"Headers"</h4>
                        <code>{"# H1"}</code>
                        <code>{"# # H2"}</code>
                        <code>{"# # # H3"}</code>
                    </div>
                    <div class="cheatsheet-item">
                        <h4>"Emphasis"</h4>
                        <code>{"**bold**"}</code>
                        <code>{"*italic*"}</code>
                        <code>{"`code`"}</code>
                    </div>
                    <div class="cheatsheet-item">
                        <h4>"Lists"</h4>
                        <code>{"- Bullet item"}</code>
                        <code>{"1. Numbered item"}</code>
                    </div>
                    <div class="cheatsheet-item">
                        <h4>"Blocks"</h4>
                        <code>{"```code block```"}</code>
                        <code>{"> blockquote"}</code>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;
    
    for line in markdown.lines() {
        // Handle code blocks
        if line.starts_with("```") {
            if in_code_block {
                html.push_str("</code></pre>");
                in_code_block = false;
            } else {
                html.push_str("<pre><code>");
                in_code_block = true;
            }
            continue;
        }
        
        if in_code_block {
            html.push_str(&escape_html(line));
            html.push_str("<br>");
            continue;
        }
        
        // Process line
        let mut processed = line.to_string();
        
        // Headers
        if processed.starts_with("# ") {
            processed = processed.replace("# ", "<h1>") + "</h1>";
        } else if processed.starts_with("## ") {
            processed = processed.replace("## ", "<h2>") + "</h2>";
        } else if processed.starts_with("### ") {
            processed = processed.replace("### ", "<h3>") + "</h3>";
        }
        
        // Blockquotes
        else if processed.starts_with("> ") {
            processed = processed.replace("> ", "<blockquote>") + "</blockquote>";
        }
        
        // Simple formatting
        processed = processed
            .replace("**", "<strong>")
            .replace("*", "<em>")
            .replace("`", "<code>");
        
        // Fix closing tags
        processed = processed
            .replace("<strong><strong>", "</strong>")
            .replace("<em><em>", "</em>")
            .replace("<code><code>", "</code>");
        
        // Lists
        if processed.starts_with("- ") {
            processed = processed.replace("- ", "• ");
        }
        
        html.push_str(&processed);
        html.push_str("<br>");
    }
    
    html
}

fn escape_html(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Markdown Editor"/>
        <MarkdownEditor/>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
