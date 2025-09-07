use handlebars::Handlebars;
use std::{fs, sync::Arc};

pub fn render(
    handlebars: &Handlebars<'_>,
    template_name: &str,
    data: &serde_json::Value,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    return Ok(handlebars.render(template_name, data)?);
}

pub fn init() -> Arc<Handlebars<'static>> {
    let mut handlebars = Handlebars::new();
    println!("Loading components from ../templates/components");
    for entry in fs::read_dir("../templates/components").expect("Failed to load components") {
        let path = entry.expect("Failed to read components").path();
        if path.extension().and_then(|s| s.to_str()) == Some("hbs") {
            let content = fs::read_to_string(&path).expect("Failed to read hole file");
            println!("Loaded component {:?}", path);

            handlebars
                .register_partial(
                    path.file_stem()
                        .expect("Expected valid file name")
                        .to_str()
                        .unwrap(),
                    content,
                )
                .expect("Failed to register component");
        }
    }

    println!("Loading pages from ../templates");
    for entry in fs::read_dir("../templates").expect("Failed to load components") {
        let path = entry.expect("Failed to read components").path();
        if path.extension().and_then(|s| s.to_str()) == Some("hbs") {
            let content = fs::read_to_string(&path).expect("Failed to read hole file");
            println!("Loaded page {:?}", path);

            handlebars
                .register_template_string(
                    path.file_stem()
                        .expect("Expected valid file name")
                        .to_str()
                        .unwrap(),
                    content,
                )
                .expect("Failed to register page");
        }
    }

    return Arc::new(handlebars);
}
