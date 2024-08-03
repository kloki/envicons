use std::env;

use bollard::{container::ListContainersOptions, Docker};
pub mod macros;

#[derive(Debug)]
pub enum Rule {
    Equals(String),
    Contains(String),
    Never,
}

impl Rule {
    pub fn apply(&self, input: &str) -> bool {
        match self {
            Rule::Never => false,
            Rule::Equals(s) => input == s,
            Rule::Contains(s) => input.contains(s),
        }
    }
}

#[derive(Debug)]
pub struct Envicon {
    icon: String,
    container: bool,
    envar: bool,
    container_rule: Rule,
    envar_rule: Rule,
}

impl Envicon {
    pub fn new(icon: String, container_rule: Rule, envar_rule: Rule) -> Self {
        Self {
            icon,
            container: false,
            envar: false,
            container_rule,
            envar_rule,
        }
    }

    pub fn apply_envar(&mut self, envar: &str) {
        if self.envar_rule.apply(envar) {
            self.envar = true
        }
    }
    pub fn apply_container(&mut self, container: &str) {
        if self.container_rule.apply(container) {
            self.container = true
        }
    }

    fn output(&self) -> String {
        match (self.container, self.envar) {
            (true, true) => format!("\x1b[32m{}\x1b[0m", self.icon),
            (true, false) => format!("\x1b[36m{}\x1b[0m", self.icon),
            (false, true) => format!("\x1b[34m{}\x1b[0m", self.icon),
            (false, false) => "".to_string(),
        }
    }
}

#[derive(Default, Debug)]
pub struct Envicons {
    items: Vec<Envicon>,
}

impl Envicons {
    pub fn new(envicons: Vec<Envicon>) -> Self {
        Self { items: envicons }
    }

    pub fn check_envars(&mut self) {
        for (key, _) in env::vars() {
            for i in self.items.iter_mut() {
                i.apply_envar(&key)
            }
        }
    }
    pub async fn check_containers(&mut self) {
        if let Ok(docker) = Docker::connect_with_local_defaults() {
            let options = Some(ListContainersOptions::<String> {
                all: false, // Only list running containers
                ..Default::default()
            });

            if let Ok(containers) = docker.list_containers(options).await {
                for container in containers {
                    if let Some(image) = container.image {
                        for i in self.items.iter_mut() {
                            i.apply_container(&image)
                        }
                    }
                }
            };
        }
    }

    pub fn output(&self) -> String {
        self.items
            .iter()
            .map(|i| i.output())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
