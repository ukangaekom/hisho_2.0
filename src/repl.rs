use crate::agents::{processing_agent::process, report_agent::report_result};
use crate::tools::tools_map::TOOLS;
use crate::tools::utils::{destructor_task, extract_tool_params};
use tokio::task::JoinSet;

/// Core execution engine that receives a chat input message,
/// processes it with the reasoning agent, matches tool calls,
/// executes tasks concurrently, and synthesizes the final report.
pub async fn execute(message: &str) -> Result<String, String> {
    if message.trim().is_empty() {
        return Err("Chat input cannot be empty.".to_string());
    }

    // 1. Process chat input with processing/reasoning agent
    let reasoning = match process(message).await {
        Ok(res) => res,
        Err(err) => return Err(format!("Processing agent error: {}", err)),
    };

    // 2. Extract structured tool tasks from reasoning result
    let agent_tasks: Vec<String> = destructor_task(&reasoning).into_iter().collect();

    // If no specific tools are requested, synthesize or return reasoning directly
    if agent_tasks.is_empty() {
        return match report_result(&reasoning).await {
            Ok(rep) => Ok(rep),
            Err(_) => Ok(reasoning),
        };
    }

    // 3. Match tools and execute tasks concurrently using tokio::task::JoinSet
    let mut tasks = JoinSet::new();

    for task_str in agent_tasks {
        tasks.spawn(async move {
            if let Some((tool, parameters)) = extract_tool_params(&task_str) {
                let params: Vec<&str> = parameters.iter().map(|s| s.as_str()).collect();

                if let Some(func) = TOOLS.get(tool.as_str()) {
                    func(&params).await
                } else {
                    format!("Tool '{}' not found", tool)
                }
            } else {
                format!("Invalid task: {}", task_str)
            }
        });
    }

    let mut results: Vec<String> = Vec::new();

    while let Some(res) = tasks.join_next().await {
        match res {
            Ok(output) => results.push(output),
            Err(e) => results.push(format!("Task execution error: {}", e)),
        }
    }

    let reply = results.join("\n");

    // 4. Report back the final synthesized response
    match report_result(&reply).await {
        Ok(result) => Ok(result),
        Err(err) => Err(format!("Report agent error: {}", err)),
    }
}