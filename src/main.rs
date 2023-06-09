use lemmy_db_schema::utils::build_db_pool;
use lemmy_server::{init_logging, start_lemmy_server};
use lemmy_utils::{error::LemmyError, settings::SETTINGS};

#[actix_web::main]
pub async fn main() -> Result<(), LemmyError> {
  init_logging(&SETTINGS.opentelemetry_url)?;
  let pool = build_db_pool(&SETTINGS).await?;
  #[cfg(not(feature = "embed-pictrs"))]
  start_lemmy_server(pool).await?;
  #[cfg(feature = "embed-pictrs")]
  {
    pict_rs::ConfigSource::memory(serde_json::json!({
        "server": {
            "address": "127.0.0.1:8080"
        },
        "old_db": {
            "path": "./pictrs/old"
        },
        "repo": {
            "type": "sled",
            "path": "./pictrs/sled-repo"
        },
        "store": {
            "type": "filesystem",
            "path": "./pictrs/files"
        }
    }))
    .init::<&str>(None)
    .expect("initialize pictrs config");
    let (lemmy, pictrs) = tokio::join!(start_lemmy_server(pool), pict_rs::run());
    lemmy?;
    pictrs.expect("run pictrs");
  }
  Ok(())
}
