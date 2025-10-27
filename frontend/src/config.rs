use once_cell::sync::OnceCell;

static CONFIG: OnceCell<Config> = OnceCell::new();

pub struct Config {
    pub api_base: String
}

impl Config {
    pub fn new() -> &'static Self {
        CONFIG.get_or_init(||
            Self {
                api_base: std::env::var("API_BASE").expect("Could not find API_BASE env var.")
            }
        )
    }
}
