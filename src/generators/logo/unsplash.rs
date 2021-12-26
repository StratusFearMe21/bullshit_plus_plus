use std::str::FromStr;

use anyhow::Context;
use dotenv_codegen::dotenv;
use tiny_skia::Pixmap;

use crate::OPTS;

const AUTHORIZATION: &str = dotenv!("UNSPLASH_API_KEY");

pub struct Unsplash;

impl Unsplash {
    pub fn photo(count: u8) -> anyhow::Result<Vec<(Pixmap, i32, i32)>> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .context("Could not build async runtime")?
            .block_on(async move {
                futures::future::try_join_all(
                    serde_json::Value::from_str(
                        &reqwest::Client::new()
                            .get(format!(
                                "https://api.unsplash.com/photos/random?query={}&count={}",
                                unsafe { OPTS.get_unchecked() }.query,
                                count
                            ))
                            .header("Accept-Version", "v1")
                            .header("Authorization", format!("Client-ID {}", AUTHORIZATION))
                            .send()
                            .await?
                            .text()
                            .await?,
                    )?
                    .as_array()
                    .context("Unsplash did not produce array")?
                    .iter()
                    .map(|i| async move {
                        let pixmap = Pixmap::decode_png(
                            &reqwest::get(format!(
                                "{}&fm=png&fit=crop&w=1080&q=80&fit=max",
                                i.get("urls")
                                    .context("Cannot get URLS feild from Unsplash")?
                                    .get("raw")
                                    .context("Failed to get raw image URL from Unsplash")?
                                    .as_str()
                                    .context("Raw image URL is not a string")?
                            ))
                            .await?
                            .bytes()
                            .await?,
                        )?;
                        let w = pixmap.width() as i32;
                        let h = pixmap.height() as i32;
                        Ok::<_, anyhow::Error>((pixmap, w, h))
                    }),
                )
                .await
            })
            .context("Failed to fetch photos from Unsplash")
    }
}
