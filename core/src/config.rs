use std::path::PathBuf;

use etcetera::{AppStrategy, AppStrategyArgs, app_strategy::choose_native_strategy};

const TOP_LEVEL_DOMAIN: &str = "net";
const AUTHOR: &str = "gw2fashionista";
const APP_NAME: &str = "GW2 Fashionista";

#[derive(Debug, Clone)]
pub struct Config {
    pub data_dir: PathBuf,
    pub db_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let strategy = choose_native_strategy(AppStrategyArgs {
            top_level_domain: TOP_LEVEL_DOMAIN.to_string(),
            author: AUTHOR.to_string(),
            app_name: APP_NAME.to_string(),
        })
        .unwrap();

        let data_dir = strategy.data_dir();
        std::fs::create_dir_all(&data_dir).unwrap();
        let db_path = data_dir.join(PathBuf::from("gw2fashionista.sqlite"));
        Self { data_dir, db_path }
    }
}
