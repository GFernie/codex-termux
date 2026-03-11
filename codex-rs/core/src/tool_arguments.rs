use crate::function_tool::FunctionCallError;
use serde_json::Value;
use serde_json::json;

pub(crate) fn normalize_exec_command_arguments(
    raw: &str,
    model: &str,
) -> Result<String, FunctionCallError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed == "null" {
        return Err(exec_command_usage_error());
    }

    let value = match serde_json::from_str::<Value>(trimmed) {
        Ok(value) => value,
        Err(_) if is_qwen_family_model(model) && !looks_like_structured_json(trimmed) => {
            Value::String(trimmed.to_string())
        }
        Err(err) if is_qwen_family_model(model) => {
            return Err(FunctionCallError::RespondToModel(format!(
                "exec_command requires JSON like {{\"cmd\":\"pwd\"}}: {err}"
            )));
        }
        Err(err) => {
            return Err(FunctionCallError::RespondToModel(format!(
                "failed to parse function arguments: {err}"
            )));
        }
    };

    let normalized = match value {
        Value::String(cmd) if !cmd.trim().is_empty() => json!({ "cmd": cmd }),
        Value::Object(mut obj) => {
            if obj.get("cmd").is_none() {
                if let Some(command) = obj.remove("command") {
                    obj.insert("cmd".to_string(), command);
                }
            }

            match obj.get("cmd").and_then(Value::as_str).map(str::trim) {
                Some(cmd) if !cmd.is_empty() => Value::Object(obj),
                _ => return Err(exec_command_usage_error()),
            }
        }
        _ => return Err(exec_command_usage_error()),
    };

    serde_json::to_string(&normalized).map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to serialize normalized exec arguments: {err}"
        ))
    })
}

pub(crate) fn normalize_mcp_tool_arguments(
    tool_name: &str,
    raw: &str,
    model: &str,
) -> Result<Option<Value>, FunctionCallError> {
    if is_memory_read_tool(tool_name) {
        return normalize_memory_read_arguments(raw, model);
    }

    parse_optional_json_arguments(raw)
}

fn normalize_memory_read_arguments(
    raw: &str,
    model: &str,
) -> Result<Option<Value>, FunctionCallError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed == "null" {
        return Err(memory_read_usage_error());
    }

    let value = match serde_json::from_str::<Value>(trimmed) {
        Ok(value) => value,
        Err(_) if is_qwen_family_model(model) && !looks_like_structured_json(trimmed) => {
            Value::String(trimmed.to_string())
        }
        Err(err) if is_qwen_family_model(model) => {
            return Err(FunctionCallError::RespondToModel(format!(
                "memory_read requires JSON like {{\"category\":\"base\"}}: {err}"
            )));
        }
        Err(err) => {
            return Err(FunctionCallError::RespondToModel(format!(
                "failed to parse function arguments: {err}"
            )));
        }
    };

    match value {
        Value::String(category) if !category.trim().is_empty() => {
            Ok(Some(json!({ "category": category })))
        }
        Value::Object(obj) => match obj.get("category").and_then(Value::as_str).map(str::trim) {
            Some(category) if !category.is_empty() => Ok(Some(Value::Object(obj))),
            _ => Err(memory_read_usage_error()),
        },
        _ => Err(memory_read_usage_error()),
    }
}

fn parse_optional_json_arguments(raw: &str) -> Result<Option<Value>, FunctionCallError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let value = serde_json::from_str::<Value>(trimmed).map_err(|err| {
        FunctionCallError::RespondToModel(format!("failed to parse function arguments: {err}"))
    })?;

    if value.is_null() {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

fn exec_command_usage_error() -> FunctionCallError {
    FunctionCallError::RespondToModel(
        "exec_command requires JSON like {\"cmd\":\"pwd\"}".to_string(),
    )
}

fn memory_read_usage_error() -> FunctionCallError {
    FunctionCallError::RespondToModel(
        "memory_read requires JSON like {\"category\":\"base\"}".to_string(),
    )
}

fn is_memory_read_tool(tool_name: &str) -> bool {
    tool_name.ends_with("memory_read")
}

fn is_qwen_family_model(model: &str) -> bool {
    let lower = model.to_ascii_lowercase();
    let model_id = lower.rsplit('/').next().unwrap_or(lower.as_str());
    model_id.starts_with("qwen") || lower.starts_with("zai-org/qwen")
}

fn looks_like_structured_json(value: &str) -> bool {
    value.starts_with('{') || value.starts_with('[') || value.starts_with('"')
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn exec_command_accepts_json_string() {
        let normalized = normalize_exec_command_arguments(r#""pwd""#, "qwen3.5-plus").unwrap();

        assert_eq!(
            serde_json::from_str::<Value>(&normalized).unwrap(),
            json!({ "cmd": "pwd" })
        );
    }

    #[test]
    fn exec_command_accepts_command_alias() {
        let normalized = normalize_exec_command_arguments(r#"{"command":"pwd"}"#, "glm-5").unwrap();

        assert_eq!(
            serde_json::from_str::<Value>(&normalized).unwrap(),
            json!({ "cmd": "pwd" })
        );
    }

    #[test]
    fn qwen_exec_command_accepts_bare_string() {
        let normalized = normalize_exec_command_arguments("pwd", "qwen3.5-plus").unwrap();

        assert_eq!(
            serde_json::from_str::<Value>(&normalized).unwrap(),
            json!({ "cmd": "pwd" })
        );
    }

    #[test]
    fn exec_command_rejects_empty_object_with_guidance() {
        let err = normalize_exec_command_arguments("{}", "qwen3.5-plus").unwrap_err();

        assert_eq!(
            err,
            FunctionCallError::RespondToModel(
                "exec_command requires JSON like {\"cmd\":\"pwd\"}".to_string()
            )
        );
    }

    #[test]
    fn memory_read_accepts_json_string() {
        let normalized =
            normalize_mcp_tool_arguments("memory_read", r#""base""#, "qwen3.5-plus").unwrap();

        assert_eq!(normalized, Some(json!({ "category": "base" })));
    }

    #[test]
    fn qwen_memory_read_accepts_bare_string() {
        let normalized =
            normalize_mcp_tool_arguments("memory_read", "base", "qwen3.5-plus").unwrap();

        assert_eq!(normalized, Some(json!({ "category": "base" })));
    }

    #[test]
    fn memory_read_rejects_missing_category_with_guidance() {
        let err = normalize_mcp_tool_arguments("memory_read", "{}", "qwen3.5-plus").unwrap_err();

        assert_eq!(
            err,
            FunctionCallError::RespondToModel(
                "memory_read requires JSON like {\"category\":\"base\"}".to_string()
            )
        );
    }

    #[test]
    fn empty_non_special_mcp_arguments_still_parse_as_none() {
        let normalized =
            normalize_mcp_tool_arguments("memory_search", " \n\t ", "qwen3.5-plus").unwrap();

        assert_eq!(normalized, None);
    }
}
