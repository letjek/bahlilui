use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;

const HTML_TEMPLATE: &str = include_str!("../contents/view.bui");
const CSS_STYLES: &str = include_str!("../contents/style.css");

struct DocsApp {
    hero_title: String,
    hero_description: String,
    features_intro: String,
    project_structure_intro: String,
    project_structure: String,
    view_bui_example: String,
    style_css_example: String,
    lib_rs_example: String,
    template_example: String,
    template_description: String,
    roadmap_intro: String,
    footer_text: String,
}

impl DocsApp {
    fn new() -> Self {
        Self {
            hero_title: "The \"No-Nonsense\" Rust → WebAssembly UI Framework".to_string(),
            hero_description: "Write UI logic in Rust, keep styling in CSS, and author templates in .bui (HTML with placeholders). Compile everything into a single .wasm bundle.".to_string(),
            features_intro: "BahlilUI is a tiny, compile-time friendly UI approach that keeps things simple and fast.".to_string(),
            project_structure_intro: "A typical BahlilUI project looks like this:".to_string(),
            project_structure: r#".
├── contents/
│   ├── view.bui
│   └── style.css
├── src/
│   ├── lib.rs
│   └── bin/
│       └── dev_server.rs
├── index.html
└── Cargo.toml"#.to_string(),
            view_bui_example: r#"<div class="card">
  <h1>{{title}}</h1>
  <p>Welcome, <strong>{{username}}</strong>!</p>
  <p>This is rendered via BahlilUI.</p>
</div>"#.to_string(),
            style_css_example: r#"body {
  font-family: sans-serif;
  background: #222;
  color: #fff;
  display: flex;
  justify-content: center;
  margin-top: 50px;
}

.card {
  background: #333;
  padding: 2rem;
  border-radius: 10px;
  border: 1px solid #dea584;
  text-align: center;
}

h1 { color: #dea584; }"#.to_string(),
            lib_rs_example: r#"use wasm_bindgen::prelude::*;
use web_sys::window;

const HTML_TEMPLATE: &str = include_str!("../contents/view.bui");
const CSS_STYLES: &str = include_str!("../contents/style.css");

struct AppData {
    title: String,
    username: String,
}

fn render(template: &str, data: &AppData) -> String {
    let mut output = template.to_string();
    output = output.replace("{{title}}", &data.title);
    output = output.replace("{{username}}", &data.username);
    output
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let document = window().expect("no window").document().expect("no document");
    let body = document.body().expect("no body");
    let head = document.head().expect("no head");

    // inject CSS
    let style = document.create_element("style")?;
    style.set_inner_html(CSS_STYLES);
    head.append_child(&style)?;

    // app data (state)
    let data = AppData {
        title: "BahlilUI".to_string(),
        username: "Developer".to_string(),
    };

    // render HTML
    let container = document.create_element("div")?;
    container.set_inner_html(&render(HTML_TEMPLATE, &data));
    body.append_child(&container)?;

    Ok(())
}"#.to_string(),
            template_example: r#"<h1>{{title}}</h1>
<p>Hello, {{username}}!</p>"#.to_string(),
            template_description: "At runtime, placeholders are replaced by strings from your Rust state.".to_string(),
            roadmap_intro: "Ideas for evolving BahlilUI into a real framework:".to_string(),
            footer_text: "Built with ❤️ in Rust + WebAssembly.".to_string(),
        }
    }
}

fn render(template: &str, data: &DocsApp) -> String {
    let mut output = template.to_string();
    output = output.replace("{{hero_title}}", &data.hero_title);
    output = output.replace("{{hero_description}}", &data.hero_description);
    output = output.replace("{{features_intro}}", &data.features_intro);
    output = output.replace("{{project_structure_intro}}", &data.project_structure_intro);
    output = output.replace("{{project_structure}}", &data.project_structure);
    output = output.replace("{{view_bui_example}}", &data.view_bui_example);
    output = output.replace("{{style_css_example}}", &data.style_css_example);
    output = output.replace("{{lib_rs_example}}", &data.lib_rs_example);
    output = output.replace("{{template_example}}", &data.template_example);
    output = output.replace("{{template_description}}", &data.template_description);
    output = output.replace("{{roadmap_intro}}", &data.roadmap_intro);
    output = output.replace("{{footer_text}}", &data.footer_text);
    output
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    log("🚀 BahlilUI Docs App Starting...");

    let document = window().expect("no window").document().expect("no document");
    let body = document.body().expect("no body");
    let head = document.head().expect("no head");

    // inject CSS
    let style = document.create_element("style")?;
    style.set_inner_html(CSS_STYLES);
    head.append_child(&style)?;

    // app data (state)
    let data = DocsApp::new();

    // render HTML
    let container = document.create_element("div")?;
    container.set_inner_html(&render(HTML_TEMPLATE, &data));
    body.append_child(&container)?;

    // Add event listeners for navigation
    setup_navigation()?;

    log("✅ BahlilUI Docs App Loaded Successfully!");
    Ok(())
}

fn setup_navigation() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();

    // Get all nav links
    let nav_links = document.query_selector_all(".nav-link")?;

    for i in 0..nav_links.length() {
        if let Some(node) = nav_links.get(i) {
            if let Ok(link) = node.dyn_into::<web_sys::Element>() {
                let link_clone = link.clone();
                let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                    if let Some(href) = link_clone.get_attribute("href") {
                        if href.starts_with('#') {
                            scroll_to_section(&href[1..]);
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                link.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
                closure.forget();
            }
        }
    }

    Ok(())
}

#[wasm_bindgen]
pub fn scroll_to_section(section_id: &str) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.get_element_by_id(section_id) {
                element.scroll_into_view();
            }
        }
    }
}
