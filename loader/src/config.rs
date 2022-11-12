use envconfig::Envconfig;

#[derive(Envconfig)]
pub(crate) struct AppConfig {
    #[envconfig(from = "DATABASE_URL")]
    pub(crate) database_url: String,
}