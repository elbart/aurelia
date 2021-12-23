use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Template parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}
