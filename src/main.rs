use envicons::{contains, envicon, equals, never, Envicons};

#[tokio::main]
async fn main() {
    let mut envicons = Envicons::new(vec![
        envicon!("󰧑", never!(), equals!("OPENAI_API_KEY")),
        envicon!("", contains!("postgres"), equals!("DATABASE_URL")),
        envicon!("", contains!("redis"), contains!("REDIS")),
        envicon!("󱩢", contains!("rabbit"), equals!("BROKER_URL")),
        envicon!("󰳃", contains!("couchdb"), contains!("COUCHDB")),
        envicon!("", contains!("mongo"), contains!("MONGO")),
        envicon!("󰐴 PX4", contains!("gazebo"), never!()),
        envicon!("󰐴 AP", contains!("ardupilot"), never!()),
        envicon!("󰐴 ", never!(), equals!("DRONE_URI")),
    ]);
    envicons.check_envars();
    envicons.check_containers().await;
    println!("{}", envicons.output());
}
