use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;
use std::sync::{Arc, OnceLock};
use eyre::Error;

static CLIENT: OnceLock<Arc<Client>> = OnceLock::new();
static PROCESS_SYSTEM_CONFIGURATION: &str = include_str!("../../knowledgebase/avalanche_gpt_report.txt");

pub fn init_env_keys() {
    if std::env::var("GEMINI_API_KEY").is_err() {
        if let Ok(Some(settings)) = crate::settings::config::AppSettings::fetch() {
            if let Some(key) = settings.gemini_api_key {
                unsafe {
                    std::env::set_var("GEMINI_API_KEY", key);
                }
            }
        }
    }
}

#[inline(always)]
pub fn get_client() -> Arc<Client> {
    CLIENT.get_or_init(|| {
        Arc::new(Client::default())
    }).clone()
}

pub async fn report_result(_text: &str) -> Result<String, Error> {
    init_env_keys();

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