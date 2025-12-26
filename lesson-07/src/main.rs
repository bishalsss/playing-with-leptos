use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::*;

#[derive(Clone, Debug)]
struct Project {
    title: String,
    description: String,
    technologies: Vec<String>,
    github_url: Option<String>,
    live_url: Option<String>,
    featured: bool,
}

#[derive(Clone, Debug)]
struct Skill {
    name: String,
    level: u8, // 0-100
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
        <Title text="John Doe - Portfolio"/>
        <Meta name="description" content="Full Stack Developer specializing in modern web technologies"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        
        <Portfolio/>
    }
}

#[component]
fn Portfolio() -> impl IntoView {
    // Hardcoded projects
    let projects = vec![
        Project {
            title: "E-Commerce Platform".to_string(),
            description: "Full-stack e-commerce with modern stack".to_string(),
            technologies: vec!["Rust".to_string(), "React".to_string(), "PostgreSQL".to_string()],
            github_url: Some("https://github.com/username/ecommerce".to_string()),
            live_url: Some("https://demo.example.com".to_string()),
            featured: true,
        },
        Project {
            title: "Task Management App".to_string(),
            description: "Collaborative task management with real-time updates".to_string(),
            technologies: vec!["TypeScript".to_string(), "Node.js".to_string(), "Socket.io".to_string()],
            github_url: Some("https://github.com/username/taskapp".to_string()),
            live_url: None,
            featured: true,
        },
        Project {
            title: "Weather Dashboard".to_string(),
            description: "Real-time weather visualization with charts".to_string(),
            technologies: vec!["Leptos".to_string(), "Rust".to_string(), "API".to_string()],
            github_url: Some("https://github.com/username/weather".to_string()),
            live_url: Some("https://weather.example.com".to_string()),
            featured: true,
        },
    ];
    
    // Hardcoded skills
    let skills = vec![
        Skill { name: "Rust".to_string(), level: 90 },
        Skill { name: "TypeScript".to_string(), level: 85 },
        Skill { name: "React".to_string(), level: 80 },
        Skill { name: "Leptos".to_string(), level: 75 },
        Skill { name: "Node.js".to_string(), level: 70 },
        Skill { name: "PostgreSQL".to_string(), level: 75 },
        Skill { name: "Docker".to_string(), level: 80 },
        Skill { name: "AWS".to_string(), level: 70 },
        Skill { name: "Git".to_string(), level: 90 },
        Skill { name: "Linux".to_string(), level: 85 },
    ];
    
    let (dark_mode, set_dark_mode) = signal(true);
    
    let toggle_dark_mode = move |_| {
        set_dark_mode.update(|mode| *mode = !*mode);
        if dark_mode.get() {
            _ = document().body().unwrap().class_list().add_1("dark");
        } else {
            _ = document().body().unwrap().class_list().remove_1("dark");
        }
    };
    
    // Set initial theme
    Effect::new(move |_| {
        if dark_mode.get() {
            _ = document().body().unwrap().class_list().add_1("dark");
        }
    });
    
    view! {
        // Navigation
        <nav class="nav">
            <div class="container">
                <div class="nav-content">
                    <a href="#home" class="logo">"JD"</a>
                    <div class="nav-links">
                        <a href="#home">"Home"</a>
                        <a href="#projects">"Projects"</a>
                        <a href="#skills">"Skills"</a>
                        <a href="#contact">"Contact"</a>
                        <button class="theme-toggle" on:click=toggle_dark_mode>
                            {move || if dark_mode.get() { "☀️" } else { "🌙" }}
                        </button>
                    </div>
                </div>
            </div>
        </nav>
        
        // Hero Section
        <section id="home" class="hero">
            <div class="container">
                <div class="hero-content">
                    <h1 class="hero-title">
                        "Hi, I'm "
                        <span class="highlight">"John Doe"</span>
                    </h1>
                    <h2 class="hero-subtitle">"Full Stack Developer & Rust Enthusiast"</h2>
                    <p class="hero-description">
                        "I build fast, scalable, and user-friendly web applications. 
                        Passionate about clean code, performance, and great user experiences."
                    </p>
                    <div class="hero-buttons">
                        <a href="#projects" class="btn btn-primary">"View Projects"</a>
                        <a href="#contact" class="btn btn-secondary">"Get in Touch"</a>
                    </div>
                </div>
                <div class="hero-image">
                    <div class="avatar">
                        <i class="fas fa-code"></i>
                    </div>
                </div>
            </div>
        </section>
        
        // Projects Section
        <section id="projects" class="section">
            <div class="container">
                <h2 class="section-title">"Featured Projects"</h2>
                <div class="projects-grid">
                    {projects.into_iter()
                        .filter(|p| p.featured)
                        .map(|project| view! { <ProjectCard project=project/> })
                        .collect_view()}
                </div>
            </div>
        </section>
        
        // Skills Section
        <section id="skills" class="section bg-alt">
            <div class="container">
                <h2 class="section-title">"Skills & Technologies"</h2>
                <div class="skills-grid">
                    {skills.into_iter().map(|skill| view! { <SkillBar skill=skill/> }).collect_view()}
                </div>
            </div>
        </section>
        
        // Contact Section
        <section id="contact" class="section">
            <div class="container">
                <h2 class="section-title">"Get In Touch"</h2>
                <div class="contact-content">
                    <div class="contact-info">
                        <div class="contact-item">
                            <i class="fas fa-envelope"></i>
                            <div>
                                <h3>Email</h3>
                                <p>john@example.com</p>
                            </div>
                        </div>
                        <div class="contact-item">
                            <i class="fas fa-map-marker-alt"></i>
                            <div>
                                <h3>Location</h3>
                                <p>San Francisco, CA</p>
                            </div>
                        </div>
                        <div class="contact-item">
                            <i class="fas fa-file"></i>
                            <div>
                                <h3>Resume</h3>
                                <a href="/resume.pdf" download>Download PDF</a>
                            </div>
                        </div>
                    </div>
                    <ContactForm/>
                </div>
            </div>
        </section>
        
        // Footer
        <footer class="footer">
            <div class="container">
                <div class="footer-content">
                    <div class="social-links">
                        <a href="https://github.com/username" target="_blank">
                            <i class="fab fa-github"></i>
                        </a>
                        <a href="https://linkedin.com/in/username" target="_blank">
                            <i class="fab fa-linkedin"></i>
                        </a>
                        <a href="https://twitter.com/username" target="_blank">
                            <i class="fab fa-twitter"></i>
                        </a>
                    </div>
                    <p class="copyright">"© 2024 John Doe. Built with Leptos & Rust."</p>
                </div>
            </div>
        </footer>
    }
}

#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    // Create links as Option<impl IntoView>
    let github_link = project.github_url.clone().map(|url| view! {
        <a href=url target="_blank" class="project-link">
            <i class="fab fa-github"></i>
            "Code"
        </a>
    });
    
    let live_link = project.live_url.clone().map(|url| view! {
        <a href=url target="_blank" class="project-link">
            <i class="fas fa-external-link-alt"></i>
            "Live Demo"
        </a>
    });
    
    view! {
        <div class="project-card">
            <div class="project-header">
                <h3 class="project-title">{project.title.clone()}</h3>
                <div class="project-tech">
                    {project.technologies.iter().map(|tech| view! { 
                        <span class="tech-tag">{tech.clone()}</span>
                    }).collect_view()}
                </div>
            </div>
            <p class="project-description">{project.description.clone()}</p>
            <div class="project-links">
                {github_link}
                {live_link}
            </div>
        </div>
    }
}

#[component]
fn SkillBar(skill: Skill) -> impl IntoView {
    view! {
        <div class="skill-item">
            <div class="skill-header">
                <span class="skill-name">{skill.name.clone()}</span>
                <span class="skill-level">{skill.level.to_string()}"%"</span>
            </div>
            <div class="skill-bar">
                <div 
                    class="skill-progress" 
                    style=move || format!("width: {}%", skill.level)
                ></div>
            </div>
        </div>
    }
}

#[component]
fn ContactForm() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (is_submitting, set_is_submitting) = signal(false);
    let (success, set_success) = signal(false);
    let (error, set_error) = signal::<Option<String>>(None);
    
    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_is_submitting.set(true);
        set_error.set(None);
        
        // Simple validation
        if name.get().is_empty() || email.get().is_empty() || message.get().is_empty() {
            set_error.set(Some("Please fill in all fields".to_string()));
            set_is_submitting.set(false);
            return;
        }
        
        // Simulate API call
        spawn_local(async move {
            // Simulate network delay
            gloo_timers::future::sleep(std::time::Duration::from_secs(1)).await;
            
            set_success.set(true);
            set_is_submitting.set(false);
            set_name.set(String::new());
            set_email.set(String::new());
            set_message.set(String::new());
            
            // Reset success message after 3 seconds
            gloo_timers::future::sleep(std::time::Duration::from_secs(3)).await;
            set_success.set(false);
        });
    };
    
    view! {
        <form class="contact-form" on:submit=handle_submit>
            <Show when=move || success.get() fallback=|| view! { <div></div> }>
                <div class="alert alert-success">
                    <i class="fas fa-check-circle"></i>
                    "Message sent successfully! I'll get back to you soon."
                </div>
            </Show>
            
            <Show when=move || error.get().is_some() fallback=|| view! { <div></div> }>
                <div class="alert alert-error">
                    <i class="fas fa-exclamation-circle"></i>
                    {move || error.get().unwrap_or_default()}
                </div>
            </Show>
            
            <div class="form-group">
                <label for="name">"Name"</label>
                <input
                    type="text"
                    id="name"
                    prop:value=name
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                    required
                />
            </div>
            
            <div class="form-group">
                <label for="email">"Email"</label>
                <input
                    type="email"
                    id="email"
                    prop:value=email
                    on:input=move |ev| set_email.set(event_target_value(&ev))
                    required
                />
            </div>
            
            <div class="form-group">
                <label for="message">"Message"</label>
                <textarea
                    id="message"
                    prop:value=message
                    on:input=move |ev| set_message.set(event_target_value(&ev))
                    rows=5
                    required
                ></textarea>
            </div>
            
            <button type="submit" class="btn btn-primary" disabled=is_submitting>
                <Show when=move || is_submitting.get() fallback=move || view! { "Send Message" }>
                    <i class="fas fa-spinner fa-spin"></i>
                    "Sending..."
                </Show>
            </button>
        </form>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
