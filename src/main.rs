use anyhow::Ok;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use ai_agent::constant::{DEEPSEEK, GLM, NEMOTRON3};
use ai_agent::llm::complete::chat_complete;
use ai_agent::llm::structured::chat_complete_structured;

#[tokio::main]
async fn main()-> anyhow::Result<()> {

    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO).finish();

    tracing::subscriber::set_global_default(subscriber)?;

   let plan = chat_complete_structured(NEMOTRON3,Some("你是一个全能助手"),
                                          "我要实现一个新的编程语言，要做什么工作？").await?;

    println!("Response:{plan:#?}");
    Ok(())
}
