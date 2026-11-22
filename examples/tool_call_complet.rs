use std::iter::chain;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use ai_agent::constant::NEMOTRON3;
use ai_agent::llm::complete::chat_complete;
use ai_agent::tools::tools;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let tools = tools();

    let plan = chat_complete(
        NEMOTRON3,
        Some("你是一个全能助手"),
        "日本的首都是哪里"
        ,tools.clone(),
    ).await?;

    tracing::info!("Response{:#?}", plan);
    println!("------------------");

    let plan = chat_complete(
        NEMOTRON3,Some("你是一个全能助手"),
        "5839乘34等于多少？",
        tools.clone()
    ).await?;

    tracing::info!("Response:{plan:#?}");


    Ok(())
}