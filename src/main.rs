extern crate tera;

use rust_embed::RustEmbed;
use std::env;
use dialoguer::{Confirm, Input, Select};
use std::fs;
use tera::Tera;
use tera::Context;
use std::path::Path;

#[derive(Debug)]
struct Config {
    project_path: String,
    template_name: String,
    docker_port_prefix: u16,
}

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Asset;

impl Config {
    fn new(args: &[String], template_list: &Vec<String>) -> Config {
        let project_path = args[1].clone();
        let template_selection = Select::new()
            .with_prompt("Template")
            .default(0)
            .items(&template_list[..])
            .interact()
            .expect("template_name input");
        let template_name = template_list[template_selection].to_string();
        let docker_port_prefix = Input::<u16>::new().with_prompt("Docker port prefix (to avoid conflicts)").interact().expect("docker_port_prefix input");

        Config { project_path, template_name, docker_port_prefix }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut root_templates: Vec<_> = Asset::iter().map(|f| {
        // TODO use something less hacky than `format!()` to fix borrow checker
        format!("{}", f.split("/").collect::<Vec<_>>().first().unwrap())
    }).collect::<Vec<_>>();
    root_templates.dedup();

    let config = Config::new(&args, &root_templates);

    let mut context = Context::new();
    context.insert("docker_port_prefix", &config.docker_port_prefix);

    if Path::new(&config.project_path[..]).exists() {
        println!("{}", &config.project_path);
        if Confirm::new().with_prompt(format!("A project already exists at path `{}`. Confirm to overwrite", &config.project_path)).interact().expect("confirm") {
            fs::remove_dir_all(&config.project_path).expect("remove project folder");
        } else {
            std::process::exit(0);
        }
    }

    let project_files = Asset::iter().filter(|f| {
        let path = Path::new(f.as_ref());

        return path.starts_with(format!("{}/", &config.template_name));
    }).collect::<Vec<_>>();

    for project_file in &project_files {
        let striped_path = Path::new(&project_file[..]).parent().unwrap().strip_prefix(format!("{}/", &config.template_name)).unwrap();
        let destination_folder = format!("{}/{}", &config.project_path, &striped_path.display());
        let destination_folder_exists = Path::new(&destination_folder[..]).exists();

        if !destination_folder_exists {
            fs::create_dir_all(&destination_folder).expect("create folder");
        }
    }

    let templates = &project_files.iter().filter(|f| f.contains(".tera")).collect::<Vec<_>>();
    for template_path in templates {
        let template = Asset::get(&template_path[..]).unwrap();
        let template_content = std::str::from_utf8(template.as_ref()).unwrap();
        let rendered_template = Tera::one_off(&template_content, &context, false).unwrap();
        let striped_path = Path::new(&template_path[..]).strip_prefix(format!("{}/", &config.template_name)).unwrap();
        let template_destination_path = format!("{}/{}", &config.project_path, &striped_path.display()).replace(".tera", "");

        fs::write(&template_destination_path, &rendered_template).expect("write");
    }

    let rest_files = &project_files.iter().filter(|f| !f.contains(".tera")).collect::<Vec<_>>();
    for file_path in rest_files {
        let file = Asset::get(&file_path[..]).unwrap();
        let file_content = std::str::from_utf8(file.as_ref()).unwrap();
        let striped_path = Path::new(&file_path[..]).strip_prefix(format!("{}/", &config.template_name)).unwrap();
        let file_destination_path = format!("{}/{}", &config.project_path, &striped_path.display());

        fs::write(&file_destination_path, &file_content).expect("write");
    }
}
