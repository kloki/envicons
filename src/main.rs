use std::{default::Default, env};

use bollard::{container::ListContainersOptions, Docker};

async fn check_containers(envicons: &mut Envicons) {
    if let Ok(docker) = Docker::connect_with_local_defaults() {
        let options = Some(ListContainersOptions::<String> {
            all: false, // Only list running containers
            ..Default::default()
        });

        if let Ok(containers) = docker.list_containers(options).await {
            for container in containers {
                if let Some(image) = container.image {
                    if image.contains("postgres") {
                        envicons.postgres.container = true
                    } else if image.contains("couchdb") {
                        envicons.couchdb.container = true
                    } else if image.contains("rabbit") {
                        envicons.rabbitmq.container = true
                    } else if image.contains("redis") {
                        envicons.redis.container = true
                    } else if image.contains("mongo") {
                        envicons.mongo.container = true
                    }
                }
            }
        };
    }
}

fn check_envars(envicons: &mut Envicons) {
    for (key, _) in env::vars() {
        if key == "DATABASE_URL" {
            envicons.postgres.envar = true
        } else if key == "OPENAI_API_KEY" {
            envicons.openai.envar = true
        } else if key == "BROKER_URL" {
            envicons.rabbitmq.envar = true
        } else if key.contains("REDIS") {
            envicons.redis.envar = true
        } else if key.contains("COUCHDB") {
            envicons.couchdb.envar = true
        }
    }
}

#[derive(Default, Debug)]
struct Item {
    container: bool,
    envar: bool,
}

impl Item {
    fn output(&self, label: &str) -> String {
        match (self.container, self.envar) {
            (true, true) => format!("\x1b[32m{}\x1b[0m", label),
            (true, false) => format!("\x1b[36m{}\x1b[0m", label),
            (false, true) => format!("\x1b[34m{}\x1b[0m", label),
            (false, false) => "".to_string(),
        }
    }
}
#[derive(Default, Debug)]
struct Envicons {
    openai: Item,
    postgres: Item,
    couchdb: Item,
    redis: Item,
    rabbitmq: Item,
    mongo: Item,
}

impl Envicons {
    fn output(&self) -> String {
        format!(
            "{}{}{}{}{}{}",
            self.openai.output("󰧑 "),
            self.postgres.output(" "),
            self.redis.output(" "),
            self.rabbitmq.output("󱩢 "),
            self.couchdb.output("󰳃 "),
            self.mongo.output(" "),
        )
    }
}

#[tokio::main]
async fn main() {
    let mut envicons = Envicons::default();
    check_containers(&mut envicons).await;
    check_envars(&mut envicons);
    println!("{}", envicons.output());
}
