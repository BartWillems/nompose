use std::error::Error;

use tera::{Context, Tera};

use crate::compose;

#[derive(RustEmbed)]
#[folder = "templates/"]
pub struct Template;

pub fn load_template() -> Result<String, Box<dyn Error>> {
    let tpl = Template::get("nomad.j2").ok_or("unable to load nomad.j2")?;

    Ok(String::from(std::str::from_utf8(tpl.as_ref())?))
}

pub fn generate_nomad_cfg(
    template_content: &str,
    service_name: &str,
    service: &compose::Service,
) -> Result<String, Box<dyn Error>> {
    let mut ctx = Context::new();
    ctx.insert("service", service);
    ctx.insert("name", service_name);

    let res = Tera::one_off(template_content, &ctx, false).unwrap();

    Ok(res)
}
