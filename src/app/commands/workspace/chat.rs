use crate::anythingllm::workspace::endpoint::ChatMode;
use crate::app::commands;
use colored::*;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use hyphenation::{Language, Load, Standard};
use textwrap::{fill, Options, WordSplitter};

/// Chat to the workspace.
///
/// A workspace has a chat mode, which can be set to either `Chat` (will not use LLM unless there
/// are relevant sources from vectorDB & does not recall chat history) or `Query` (uses LLM general
/// knowledge w/custom embeddings to produce output, uses rolling chat history).
pub async fn chat(workspace_name: String, chat_mode: ChatMode) -> eyre::Result<()> {
    let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
    let options = Options::new(120).word_splitter(WordSplitter::Hyphenation(dictionary));
    let client = commands::anythingllm_client();
    let workspace = client.get_workspace_by_name(&workspace_name).await;
    if workspace.is_err() {
        return Err(eyre::eyre!("Workspace not found"));
    }

    let mut chat_mode = chat_mode;

    println!("{}", "Ask your Documents Anything".bold());

    loop {
        println!();
        let query = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Prompt")
            .interact_text()
            .unwrap();

        match query.as_str() {
            "/query" => {
                chat_mode = ChatMode::Query;
                client
                    .post_workspace_slug_chat(&workspace.as_ref().unwrap().slug, &query, &chat_mode)
                    .await?;
                println!("{}", "Query mode set".dimmed());
                continue;
            }
            "/chat" => {
                chat_mode = ChatMode::Chat;
                client
                    .post_workspace_slug_chat(&workspace.as_ref().unwrap().slug, &query, &chat_mode)
                    .await?;
                println!("{}", "Chat mode set".dimmed());
                continue;
            }
            "/exit" => break,
            _ => {}
        }

        let response = client
            .post_workspace_slug_chat(&workspace.as_ref().unwrap().slug, &query, &chat_mode)
            .await?;

        let wrapped = fill(&response.text_response, &options);
        println!("{}", wrapped);

        if !response.sources.is_empty() {
            println!("\n{}", "Sources:".bold());
            for source in response.sources {
                println!(" - {}", source.title);
            }
        }
    }

    Ok(())
}
