use crate::gaia::models::{GaiaRow, HfResponse};

pub async fn load_gaia_level1() -> anyhow::Result<Vec<GaiaRow>>{
    let client= reqwest::Client::new();
    // https://www.modelscope.cn/api/v1/datasets/preview/rows?Owner=gaia-benchmark&Name=GAIA&Config=2023_level1&Split=validation&Page=1&PageSize=100
    let response = client
        .get("https://www.modelscope.cn/api/v1/datasets/preview/rows")
        .query(&[
            ("Owner","gaia-benchmark"),
            ("Name","GAIA"),
            ("Config","2023_level1"),
            ("Split","validation"),
            ("Page","1"),
            ("PageSize","2"),
        ])
        .send()
        .await?
        .json::<HfResponse>()
        .await?;
    Ok(response.rows.into_iter().map(|r| r).collect())
}