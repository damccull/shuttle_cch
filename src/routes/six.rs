use anyhow::Context;

use actix_web::{post, HttpResponse, Responder};
use fancy_regex::Regex;
use serde::{Deserialize, Serialize};

#[tracing::instrument]
#[post("/6")]
pub async fn elf_on_a_self(input: String) -> impl Responder {
    let result = match_elfs(&input);
    let Ok(result) = result else {
        tracing::error!("Unable to parse input: {:#?}", result);
        return HttpResponse::UnprocessableEntity().body("Unable to process your input");
    };
    HttpResponse::Ok().json(&result)
}

#[tracing::instrument]
fn match_elfs<'i>(input: &'i str) -> Result<ElfReply, anyhow::Error> {
    tracing::debug!("Building regex matchers");
    let elf_matcher = Regex::new(r#"elf"#).context("Couldn't compile regex")?;
    let elf_shelf_matcher = Regex::new(r#"elf on a shelf"#).context("Couldn't compile regex")?;
    let empty_shelf_matcher =
        Regex::new(r#"(?<!elf on a )shelf"#).context("Couldn't compile regex")?;

    let elfs = elf_matcher.find_iter(input).count().try_into().unwrap_or(0);
    let elfs_on_shelfs = elf_shelf_matcher
        .find_iter(input)
        .count()
        .try_into()
        .unwrap_or(0);
    let shelfs_without_elfs = empty_shelf_matcher
        .find_iter(input)
        .count()
        .try_into()
        .unwrap_or(0);
    tracing::debug!(
        "Matches: Elfs: {:?}, ElfsOnShelfs: {:?}, ShelfsWithoutElfs: {:?}",
        &elfs,
        &elfs_on_shelfs,
        &shelfs_without_elfs
    );
    Ok(ElfReply {
        elf: elfs,
        elf_on_a_shelf: elfs_on_shelfs,
        shelf_with_no_elf_on_it: shelfs_without_elfs,
    })
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ElfReply {
    pub elf: u32,
    #[serde(rename = "elf on a shelf")]
    pub elf_on_a_shelf: u32,
    #[serde(rename = "shelf with no elf on it")]
    pub shelf_with_no_elf_on_it: u32,
}
