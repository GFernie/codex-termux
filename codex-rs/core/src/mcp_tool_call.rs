use std::time::Instant;

use tracing::error;

use crate::codex::Session;
use crate::codex::TurnContext;
use crate::function_tool::FunctionCallError;
use crate::protocol::EventMsg;
use crate::protocol::McpInvocation;
use crate::protocol::McpToolCallBeginEvent;
use crate::protocol::McpToolCallEndEvent;
use crate::tool_arguments::normalize_mcp_tool_arguments;
use codex_protocol::models::FunctionCallOutputPayload;
use codex_protocol::models::ResponseInputItem;

/// Handles the specified tool call dispatches the appropriate
/// `McpToolCallBegin` and `McpToolCallEnd` events to the `Session`.
pub(crate) async fn handle_mcp_tool_call(
    sess: &Session,
    turn_context: &TurnContext,
    call_id: String,
    server: String,
    tool_name: String,
    arguments: String,
) -> ResponseInputItem {
    let model = turn_context.client.get_model();
    let arguments_value = match normalize_mcp_tool_arguments(&tool_name, &arguments, &model) {
        Ok(value) => value,
        Err(FunctionCallError::RespondToModel(content)) => {
            error!("failed to normalize tool call arguments for {tool_name}: {content}");
            return ResponseInputItem::FunctionCallOutput {
                call_id: call_id.clone(),
                output: FunctionCallOutputPayload {
                    content,
                    success: Some(false),
                    ..Default::default()
                },
            };
        }
        Err(err) => {
            error!("failed to normalize tool call arguments for {tool_name}: {err}");
            return ResponseInputItem::FunctionCallOutput {
                call_id: call_id.clone(),
                output: FunctionCallOutputPayload {
                    content: format!("err: {err}"),
                    success: Some(false),
                    ..Default::default()
                },
            };
        }
    };

    let invocation = McpInvocation {
        server: server.clone(),
        tool: tool_name.clone(),
        arguments: arguments_value.clone(),
    };

    let tool_call_begin_event = EventMsg::McpToolCallBegin(McpToolCallBeginEvent {
        call_id: call_id.clone(),
        invocation: invocation.clone(),
    });
    notify_mcp_tool_call_event(sess, turn_context, tool_call_begin_event).await;

    let start = Instant::now();
    // Perform the tool call.
    let result = sess
        .call_tool(&server, &tool_name, arguments_value.clone())
        .await
        .map_err(|e| format!("tool call error: {e:?}"));
    if let Err(e) = &result {
        tracing::warn!("MCP tool call error: {e:?}");
    }
    let tool_call_end_event = EventMsg::McpToolCallEnd(McpToolCallEndEvent {
        call_id: call_id.clone(),
        invocation,
        duration: start.elapsed(),
        result: result.clone(),
    });

    notify_mcp_tool_call_event(sess, turn_context, tool_call_end_event.clone()).await;

    ResponseInputItem::McpToolCallOutput { call_id, result }
}

async fn notify_mcp_tool_call_event(sess: &Session, turn_context: &TurnContext, event: EventMsg) {
    sess.send_event(turn_context, event).await;
}
