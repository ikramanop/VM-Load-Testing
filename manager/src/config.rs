use envconfig::Envconfig;

#[derive(Envconfig)]
pub(crate) struct AppConfig {
    #[envconfig(from = "DATABASE_URL")]
    pub(crate) database_url: String,

    #[envconfig(from = "POLL_OVER_QUEUE", default = "1")]
    pub(crate) poll: i8,
}