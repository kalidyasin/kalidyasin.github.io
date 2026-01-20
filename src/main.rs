use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub url: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct Contact {
    pub email: String,
    pub github: String,
    pub linkedin: String,
    pub twitter: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct WorkExperience {
    pub company: String,
    pub role: String,
    pub period: String,
    pub description: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct PortfolioContent {
    pub welcome: String,
    pub motd: String,
    pub about: String,
    pub skills: Vec<String>,
    pub projects: Vec<Project>,
    pub contact: Contact,
    pub technologies: Vec<String>,
    pub work_experience: Vec<WorkExperience>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TerminalLine {
    Input(String),
    Output(Html),
    Error(String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum ConfirmAction {
    Restart,
    PowerOff,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum AppMode {
    #[default]
    Normal,
    Confirm(ConfirmAction),
    PowerOff,
}

const COMMANDS: &[(&str, &str)] = &[
    ("about", "Learn more about me"),
    ("skills", "List my professional skills"),
    ("projects", "View my featured projects"),
    ("contact", "Get my contact information"),
    ("technologies", "Tools and platforms I use"),
    ("work-experience", "My professional journey"),
    ("help", "Show this help message"),
    ("clear", "Clear the terminal screen"),
    ("restart", "Reboot the terminal session"),
    ("poweroff", "Shut down the system"),
];

#[function_component(App)]
pub fn app() -> Html {
    let content = use_state(PortfolioContent::default);
    let lines = use_state(Vec::<TerminalLine>::new);
    let history = use_state(Vec::<String>::new);
    let history_index = use_state(|| -1i32);
    let input_value = use_state(String::new);
    let app_mode = use_state(AppMode::default);
    let input_ref = use_node_ref();

    {
        let content = content.clone();
        let lines = lines.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let resp = match Request::get("public/content.json").send().await {
                    Ok(r) => r,
                    Err(e) => {
                        lines.set(vec![TerminalLine::Error(format!("Failed to fetch content: {}", e))]);
                        return;
                    }
                };
                
                let fetched: Result<PortfolioContent, _> = resp.json().await;
                match fetched {
                    Ok(data) => {
                        lines.set(vec![
                            TerminalLine::Output(html! { <div class="text-tokyonight-green font-bold whitespace-pre-wrap">{&data.welcome}</div> }),
                            TerminalLine::Output(html! { <div class="text-tokyonight-yellow italic whitespace-pre-wrap">{&data.motd}</div> }),
                        ]);
                        content.set(data);
                    }
                    Err(e) => {
                        lines.set(vec![TerminalLine::Error(format!("Failed to parse content: {}", e))]);
                    }
                }
            });
            || ()
        });
    }

    {
        let input_ref = input_ref.clone();
        use_effect_with((), move |_| {
            let input_ref = input_ref.clone();
            let timeout = gloo_timers::callback::Timeout::new(100, move || {
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    let _ = input.focus();
                }
            });
            move || {
                timeout.cancel();
            }
        });
    }

    let execute_command = {
        let lines = lines.clone();
        let history = history.clone();
        let history_index = history_index.clone();
        let app_mode = app_mode.clone();
        let content = content.clone();

        Callback::from(move |cmd: String| {
            let cmd_trimmed = cmd.trim();
            if cmd_trimmed.is_empty() {
                let mut new_lines = (*lines).clone();
                new_lines.push(TerminalLine::Input(String::new()));
                lines.set(new_lines);
                return;
            }

            let mut new_history = (*history).clone();
            new_history.push(cmd_trimmed.to_string());
            history.set(new_history);
            history_index.set(-1);

            let mut new_lines = (*lines).clone();
            new_lines.push(TerminalLine::Input(cmd_trimmed.to_string()));

            match cmd_trimmed.to_lowercase().as_str() {
                "help" => {
                    new_lines.push(TerminalLine::Output(html! {
                        <div class="flex flex-col space-y-1 my-2">
                            {for COMMANDS.iter().map(|(c, d)| html! {
                                <div class="flex flex-wrap">
                                    <span class="text-tokyonight-blue font-bold w-40 flex-shrink-0">{c}</span>
                                    <span class="text-tokyonight-fg opacity-80">{"- "}{d}</span>
                                </div>
                            })}
                        </div>
                    }));
                }
                "about" => {
                    new_lines.push(TerminalLine::Output(html! {
                        <div class="my-2 whitespace-pre-wrap">{&content.about}</div>
                    }));
                }
                "skills" => {
                    new_lines.push(TerminalLine::Output(html! {
                        <div class="flex flex-wrap gap-2 my-2">
                            {for content.skills.iter().map(|s| html! {
                                <span class="px-2 py-1 bg-tokyonight-black rounded text-tokyonight-green">{s}</span>
                            })}
                        </div>
                    }));
                }
                "projects" => {
                    new_lines.push(TerminalLine::Output(html! {
                        <div class="space-y-4 my-2">
                            {for content.projects.iter().map(|p| html! {
                                <div>
                                    <a href={p.url.clone()} target="_blank" class="text-tokyonight-blue font-bold hover:underline">
                                        {&p.title}
                                    </a>
                                    <p class="text-sm opacity-80 whitespace-pre-wrap">{&p.description}</p>
                                </div>
                            })}
                        </div>
                    }));
                }
                "contact" => {
                    new_lines.push(TerminalLine::Output(html! {
                        <div class="space-y-1 my-2">
                            <div><span class="text-tokyonight-magenta">{"email: "}</span><a href={format!("mailto:{}", content.contact.email)}>{&content.contact.email}</a></div>
                            <div><span class="text-tokyonight-magenta">{"github: "}</span><a href={content.contact.github.clone()} target="_blank">{&content.contact.github}</a></div>
                            <div><span class="text-tokyonight-magenta">{"linkedin: "}</span><a href={content.contact.linkedin.clone()} target="_blank">{&content.contact.linkedin}</a></div>
                            <div><span class="text-tokyonight-magenta">{"twitter: "}</span><a href={content.contact.twitter.clone()} target="_blank">{&content.contact.twitter}</a></div>
                        </div>
                    }));
                }
                "technologies" => {
                    new_lines.push(TerminalLine::Output(html! {
                        <div class="flex flex-wrap gap-2 my-2">
                            {for content.technologies.iter().map(|t| html! {
                                <span class="px-2 py-1 border border-tokyonight-bright_black rounded text-tokyonight-cyan">{t}</span>
                            })}
                        </div>
                    }));
                }
                "work-experience" => {
                    new_lines.push(TerminalLine::Output(html! {
                        <div class="space-y-4 my-2">
                            {for content.work_experience.iter().map(|w| html! {
                                <div class="border-l-2 border-tokyonight-bright_black pl-4">
                                    <div class="flex justify-between items-center">
                                        <span class="text-tokyonight-yellow font-bold">{&w.role}</span>
                                        <span class="text-xs opacity-60">{&w.period}</span>
                                    </div>
                                    <div class="text-tokyonight-magenta text-sm">{&w.company}</div>
                                    <p class="text-sm mt-1 whitespace-pre-wrap">{&w.description}</p>
                                </div>
                            })}
                        </div>
                    }));
                }
                "clear" => {
                    lines.set(vec![]);
                    return;
                }
                "restart" => {
                    app_mode.set(AppMode::Confirm(ConfirmAction::Restart));
                    lines.set(new_lines);
                    return;
                }
                "poweroff" => {
                    app_mode.set(AppMode::Confirm(ConfirmAction::PowerOff));
                    lines.set(new_lines);
                    return;
                }
                _ => {
                    new_lines.push(TerminalLine::Error(format!("Command not found: {}", cmd_trimmed)));
                }
            }
            lines.set(new_lines);
        })
    };

    let on_keydown = {
        let input_value = input_value.clone();
        let history = history.clone();
        let history_index = history_index.clone();
        let app_mode = app_mode.clone();
        let execute_command = execute_command.clone();
        let lines = lines.clone();
        let content = content.clone();

        Callback::from(move |e: KeyboardEvent| {
            if let AppMode::Confirm(action) = &*app_mode {
                if e.key() == "Enter" {
                    e.prevent_default();
                    let target: HtmlInputElement = e.target_unchecked_into();
                    let val = target.value().trim().to_lowercase();
                    target.set_value("");
                    input_value.set(String::new());
                    if val == "y" || val == "yes" {
                        match action {
                            ConfirmAction::Restart => {
                                lines.set(vec![
                                    TerminalLine::Output(html! { <div class="text-tokyonight-green font-bold whitespace-pre-wrap">{&content.welcome}</div> }),
                                    TerminalLine::Output(html! { <div class="text-tokyonight-yellow italic whitespace-pre-wrap">{&content.motd}</div> }),
                                ]);
                            }
                            ConfirmAction::PowerOff => app_mode.set(AppMode::PowerOff),
                        }
                        if !matches!(*action, ConfirmAction::PowerOff) {
                            app_mode.set(AppMode::Normal);
                        }
                    } else {
                        app_mode.set(AppMode::Normal);
                    }
                    return;
                }
                if e.key() == "Escape" {
                    e.prevent_default();
                    input_value.set(String::new());
                    app_mode.set(AppMode::Normal);
                    return;
                }
                // Let other keys bubble up/pass through to the input field
                return;
            }

            match e.key().as_str() {
                "Enter" => {
                    e.prevent_default();
                    let target: HtmlInputElement = e.target_unchecked_into();
                    let cmd = target.value();
                    target.set_value("");
                    input_value.set(String::new());
                    execute_command.emit(cmd);
                }
                "ArrowUp" => {
                    e.prevent_default();
                    if !history.is_empty() {
                        let target: HtmlInputElement = e.target_unchecked_into();
                        let new_index = if *history_index == -1 {
                            (history.len() as i32) - 1
                        } else {
                            (*history_index - 1).max(0)
                        };
                        history_index.set(new_index);
                        let val = history[new_index as usize].clone();
                        target.set_value(&val);
                        input_value.set(val);
                    }
                }
                "ArrowDown" => {
                    e.prevent_default();
                    if *history_index != -1 {
                        let target: HtmlInputElement = e.target_unchecked_into();
                        let new_index = *history_index + 1;
                        if new_index >= (history.len() as i32) {
                            history_index.set(-1);
                            target.set_value("");
                            input_value.set(String::new());
                        } else {
                            history_index.set(new_index);
                            let val = history[new_index as usize].clone();
                            target.set_value(&val);
                            input_value.set(val);
                        }
                    }
                }
                "Tab" => {
                    e.prevent_default();
                    let target: HtmlInputElement = e.target_unchecked_into();
                    let current_input = target.value().to_lowercase();
                    if let Some((matched, _)) = COMMANDS.iter().find(|(c, _)| c.starts_with(&current_input)) {
                        target.set_value(matched);
                        input_value.set(matched.to_string());
                    }
                }
                _ => {}
            }
        })
    };

    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };

    let focus_input = {
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
        })
    };

    if matches!(*app_mode, AppMode::PowerOff) {
        return html! {
            <div class="fixed inset-0 bg-black flex items-center justify-center text-white font-mono p-4 text-center">
                <div>
                    <div class="mb-4 text-2xl font-bold">{"SYSTEM HALTED"}</div>
                    <div>{"The terminal has been powered off safely."}</div>
                    <div class="mt-8 opacity-50">{"Refresh the page to reboot."}</div>
                </div>
            </div>
        };
    }

    let container_ref = use_node_ref();

    {
        let lines = lines.clone();
        let container_ref = container_ref.clone();
        let input_ref = input_ref.clone();
        use_effect_with(lines, move |_| {
            if let Some(div) = container_ref.cast::<web_sys::HtmlElement>() {
                div.set_scroll_top(div.scroll_height());
            }
            
            let input_ref = input_ref.clone();
            let timeout = gloo_timers::callback::Timeout::new(50, move || {
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    let _ = input.focus();
                }
            });
            
            move || {
                timeout.cancel();
            }
        });
    }

    html! {
        <div 
            class="h-screen w-screen font-mono text-tokyonight-fg bg-tokyonight-bg cursor-text overflow-hidden p-4 flex flex-col outline-none" 
            onclick={focus_input}
        >
            <div ref={container_ref} class="w-full h-full flex-grow overflow-hidden mb-4">
                {for lines.iter().map(|line| {
                    match line {
                        TerminalLine::Input(text) => html! {
                            <div class="flex">
                                <span class="text-tokyonight-magenta mr-2">{"❯"}</span>
                                <span>{text}</span>
                            </div>
                        },
                        TerminalLine::Output(h) => h.clone(),
                        TerminalLine::Error(err) => html! {
                            <div class="text-tokyonight-red font-bold whitespace-pre-wrap">{"Error: "}{err}</div>
                        },
                    }
                })}

                <div class="flex mt-1 pb-4 flex-wrap">
                    if let AppMode::Confirm(action) = &*app_mode {
                        <span class="text-tokyonight-bright_yellow mr-2">
                            {match action {
                                ConfirmAction::Restart => "Are you sure you want to restart the terminal? (y/n) ",
                                ConfirmAction::PowerOff => "Are you sure you want to power off the system? Refresh to reuse terminal (y/n) ",
                            }}
                        </span>
                    } else {
                        <>
                            <span class="w-full text-tokyonight-cyan mr-2">{"~"}</span>
                            <span class="text-tokyonight-magenta mr-2">{"❯"}</span>
                        </>
                    }
                    <input
                        ref={input_ref}
                        class="bg-transparent border-none outline-none flex-grow text-tokyonight-fg min-w-[50px]"
                        type="text"
                        value={(*input_value).clone()}
                        oninput={on_input}
                        onkeydown={on_keydown}
                        autofocus=true
                        autocomplete="off"
                        spellcheck="false"
                    />
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_content() {
        let json = r#"{
            "welcome": "Welcome",
            "motd": "MOTD",
            "about": "About",
            "skills": ["Rust"],
            "projects": [{"title": "P1", "description": "D1", "url": "U1"}],
            "contact": {"email": "E1", "github": "G1", "linkedin": "L1", "twitter": "T1"},
            "technologies": ["T1"],
            "work_experience": [{"company": "C1", "role": "R1", "period": "P1", "description": "D1"}]
        }"#;
        let content: PortfolioContent = serde_json::from_str(json).unwrap();
        assert_eq!(content.welcome, "Welcome");
        assert_eq!(content.skills.len(), 1);
        assert_eq!(content.projects[0].title, "P1");
    }

    #[test]
    fn test_commands_list() {
        assert!(COMMANDS.iter().any(|(c, _)| *c == "help"));
        assert!(COMMANDS.iter().any(|(c, _)| *c == "about"));
        assert!(COMMANDS.iter().any(|(c, _)| *c == "clear"));
    }
}
