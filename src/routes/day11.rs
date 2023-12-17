use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, HttpRequest};
use anyhow::Context;

use crate::CodehuntError;

#[tracing::instrument]
#[get("/11/assets/{filename:.*}")]
pub async fn static_asset(request: HttpRequest) -> Result<NamedFile, CodehuntError> {
    let base_dir = std::env::current_dir()
        .context("Couldn't find current working directory")?
        .join("static");
    let path: PathBuf = request
        .match_info()
        .query("filename")
        .parse()
        .context("Unable to get filename from request query")?;
    let path = base_dir.join(path);
    tracing::debug!("Built path: {:#?}", path);
    let file = NamedFile::open(path).context("Could not open file")?;
    Ok(file)
}
