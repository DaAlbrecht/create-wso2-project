use anyhow::Context;
use rust_embed::RustEmbed;
use std::{
    fmt::{self, Display, Formatter},
    fs, path,
};

#[derive(Debug, Clone, Copy)]
pub enum ProjectTemplate {
    Apim,
    Migw,
    Mi,
}

#[derive(Debug, Clone, Copy)]
pub enum Flavor {
    Docker,
}

#[derive(RustEmbed)]
#[folder = "fragments"]
#[allow(clippy::upper_case_acronyms)]
struct FRAGMENTS;

impl Display for ProjectTemplate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Apim => write!(f, "API Manager"),
            Self::Mi => write!(f, "Micro Integrator"),
            Self::Migw => write!(f, "Microgateway"),
        }
    }
}

impl Flavor {
    pub const fn slect_text<'a>(&self) -> &'a str {
        match self {
            Self::Docker => "Docker",
        }
    }
}

impl Display for Flavor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Docker => write!(f, "Docker"),
        }
    }
}

impl ProjectTemplate {
    pub const ALL: [Self; 3] = [Self::Apim, Self::Mi, Self::Migw];

    pub const fn flavors(&self) -> &[Flavor] {
        match self {
            Self::Apim => &[Flavor::Docker],
            Self::Mi => &[Flavor::Docker],
            Self::Migw => &[Flavor::Docker],
        }
    }

    pub fn render(&self, flavor: &Flavor, target_dir: &std::path::Path) -> anyhow::Result<()> {
        let prefix = format!("fragment-{self}-{flavor}", self = self, flavor = flavor);

        let manifest_bytes = FRAGMENTS::get(&format!("{}/pom.xml", prefix))
            .with_context(|| "Failed to get manifest bytes")?
            .data;

        let manifest_str = String::from_utf8(manifest_bytes.to_vec())?;

        let project_name = target_dir.file_name().unwrap().to_string_lossy();

        let manifest_str = self.replace_vars(
            &manifest_str,
            "~groupId~",
            &format!("com.example.{}", project_name),
        );

        let manifest_str = self.replace_vars(&manifest_str, "~artifactId~", &project_name);
        let manifest_str = self.replace_vars(&manifest_str, "~name~", &project_name);

        fs::write(target_dir.join("pom.xml"), &manifest_str)?;

        let files = FRAGMENTS::iter()
            .filter(|f| {
                f.to_string().starts_with(&format!("{}", prefix))
                    && !f.to_string().ends_with("pom.xml")
            })
            .map(|f| f.to_string())
            .collect::<Vec<_>>();

        for file in files {
            let data = FRAGMENTS::get(&file)
                .with_context(|| format!("Failed to get file {}", file))?
                .data;

            // remove the first component, which is certainly the fragment directory they were in before getting embeded into the binary
            let p = path::PathBuf::from(file)
                .components()
                .skip(1)
                .collect::<path::PathBuf>();

            let p = target_dir.join(p);
            let file_name = p.file_name().unwrap();

            let parent = p.parent().unwrap();
            fs::create_dir_all(parent)?;
            fs::write(parent.join(file_name), &data)?;
        }

        Ok(())
    }

    fn replace_vars(&self, file: &str, placeholder: &str, r: &str) -> String {
        file.replace(placeholder, r)
    }
}
