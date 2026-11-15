use ai_agent::constant::{DEEPSEEK, GLM, NEMOTRON3};
use ai_agent::llm::semaphore::get_semaphore;
use ai_agent::llm::stream::{ chat_stream_with_retry};
use futures::StreamExt;
use tokio::task::{JoinError, JoinSet};
use tracing::{Instrument, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let prompts = vec![
        "用三句话解释Rust所有权机制",
        "什么是异步编程，和多线程有什么区别",
        "解释一下TCP三次握手的过程",
        "用简单的话说明什么是大语言模型",
        "Rust 中Arc 和 Rc 的区别是什么",
        "什么是RAG，为什么AI的应用里常用它",
        "解释HTTP和HTTPS的区别",
        "什么是死锁，怎么避免",
        "用生活比喻什么是递归",
        "为什么rust没有Gc但是内存安全",
    ];

    let mut set = JoinSet::new();
    for prompt in prompts {
        let span = tracing::info_span!("Chat", prompt = prompt);
        set.spawn(
            async move {
                tracing::info!("\n\n{}", prompt);
                let permit = get_semaphore().acquire().await?;
                let output = chat_stream_with_retry(NEMOTRON3, Some("你是一个全能助手"), prompt).await?;
                drop(permit);
                Ok::<_, anyhow::Error>((prompt, output))
            }
            .instrument(span),
        );
    }

    while let Some(result) = set.join_next().await {
        match result {
            Ok(Ok((prompt, result))) => tracing::info!("\n{}\n{}", prompt, result),
            Ok(Err(e)) => tracing::error!("Task panicked:{} ", e),
            Err(err) => tracing::error!("Task panicked:{err}"),
        }
    }

    Ok(())
}
