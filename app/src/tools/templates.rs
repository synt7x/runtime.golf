use handlebars::Handlebars;
use std::fs;

pub fn render(
    template_path: &str,
    data: &serde_json::Value,
) -> Result<String, Box<dyn std::error::Error>> {
    let template_content = fs::read_to_string(template_path)?;
    let mut handlebars = Handlebars::new();

    handlebars.register_template_string("template", template_content)?;
    let rendered = handlebars.render("template", data)?;

    Ok(rendered)
}
