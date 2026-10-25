use anyhow::Ok;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use crate::constant::{DEEPSEEK, GEMMA, HY3, NEMOTRON3, QWEN, XING};
use crate::llm::complete::chat_complete;

mod llm;
mod constant;

#[tokio::main]
async fn main()-> anyhow::Result<()> {

    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO).finish();

    tracing::subscriber::set_global_default(subscriber)?;

   let content = chat_complete(DEEPSEEK,Some("你是一个全能助手"),"足球冠军是谁？").await?;

    println!("Response:{content}");
    Ok(())
}
