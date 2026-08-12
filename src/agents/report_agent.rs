use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;
use std::sync::Arc;
use eyre::Error;

static PROCESS_SYSTEM_CONFIGURATION: &str = include_str!("../../knowledgebase/avalanche_gpt_report.txt");


#[inline(always)]
pub fn get_client() -> Arc<Client> {
    Arc::new(Client::default())
}

pub async fn report_result(_text: &str) -> Result<String, Error> {

    let client = get_client();
    let chat_req: ChatRequest = ChatRequest::new(vec![
        ChatMessage::system(PROCESS_SYSTEM_CONFIGURATION),
        ChatMessage::user(_text),
    ]);

    let model: &str = "gemini-2.5-flash";

    let chat_res = client.exec_chat(model, chat_req, None).await;

    match chat_res {
        Ok(res) => Ok(res.into_first_text().unwrap_or_default()),
        Err(err) => {
            eprintln!("Failed to execute chat: {:?}", err);
            Ok(String::from("Unable to get reports"))
        }
    }
}