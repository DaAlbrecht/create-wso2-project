mod template;
use std::fs;

use dialoguer::{console::Term, theme::ColorfulTheme, Confirm, Select};
use template::ProjectTemplate;

pub fn run() -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;

    let project_name = dialoguer::Input::<String>::new()
        .with_prompt("Project Name")
        .default("demo".to_string())
        .interact_text()
        .unwrap()
        .trim()
        .to_string();

    let target_dir = cwd.join(project_name);

    //check if project name is valid in pom, if so use it otherwise convert to valid name

    //check if target dir exists, if so ask to overwrite or exit
    //
    let templates = ProjectTemplate::ALL.to_vec();

    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("choose WSO2 product")
        .items(&templates)
        .default(0)
        .interact()
        .unwrap();

    let template = templates[index];

    let flavors = template.flavors();

    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("choose flavor")
        .items(&flavors.iter().map(|f| f.slect_text()).collect::<Vec<_>>())
        .default(0)
        .interact()
        .unwrap();

    let flavor = flavors[index];

    if target_dir.exists() {
        let term = Term::stderr();
        term.write_line("Project already exists. Do you want to overwrite it?")?;
        if Confirm::new()
            .with_prompt("Overwrite?")
            .default(false)
            .interact()?
        {
            fs::remove_dir_all(&target_dir)?;
            fs::create_dir_all(&target_dir).expect("Failed to create project directory");
        } else {
            std::process::exit(0);
        }
    } else {
        fs::create_dir_all(&target_dir).expect("Failed to create project directory");
    }

    //create project files

    template.render(&flavor, &target_dir)?;

    Ok(())
}
