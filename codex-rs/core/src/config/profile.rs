use codex_utils_absolute_path::AbsolutePathBuf;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use crate::protocol::AskForApproval;
use codex_protocol::config_types::ReasoningSummary;
use codex_protocol::config_types::SandboxMode;
use codex_protocol::config_types::Verbosity;
use codex_protocol::openai_models::ReasoningEffort;

/// Collection of common configuration options that a user can define as a unit
/// in `config.toml`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfigProfile {
    pub model: Option<String>,
    /// The key in the `model_providers` map identifying the
    /// [`ModelProviderInfo`] to use.
    pub model_provider: Option<String>,
    pub model_context_window: Option<i64>,
    pub model_auto_compact_token_limit: Option<i64>,
    pub approval_policy: Option<AskForApproval>,
    pub sandbox_mode: Option<SandboxMode>,
    pub model_reasoning_effort: Option<ReasoningEffort>,
    pub model_reasoning_summary: Option<ReasoningSummary>,
    pub model_verbosity: Option<Verbosity>,
    pub chatgpt_base_url: Option<String>,
    pub experimental_instructions_file: Option<AbsolutePathBuf>,
    pub experimental_compact_prompt_file: Option<AbsolutePathBuf>,
    pub include_apply_patch_tool: Option<bool>,
    pub experimental_use_unified_exec_tool: Option<bool>,
    pub experimental_use_freeform_apply_patch: Option<bool>,
    pub tools_web_search: Option<bool>,
    pub tools_view_image: Option<bool>,
    pub analytics: Option<crate::config::types::AnalyticsConfigToml>,
    /// Optional feature toggles scoped to this profile.
    #[serde(default)]
    pub features: Option<crate::features::FeaturesToml>,
    pub oss_provider: Option<String>,
}

impl From<ConfigProfile> for codex_app_server_protocol::Profile {
    fn from(config_profile: ConfigProfile) -> Self {
        Self {
            model: config_profile.model,
            model_provider: config_profile.model_provider,
            approval_policy: config_profile.approval_policy,
            model_reasoning_effort: config_profile.model_reasoning_effort,
            model_reasoning_summary: config_profile.model_reasoning_summary,
            model_verbosity: config_profile.model_verbosity,
            chatgpt_base_url: config_profile.chatgpt_base_url,
        }
    }
}

const CODING_CONTEXT_WINDOW: i64 = 128_000;
const CODING_AUTO_COMPACT_TOKEN_LIMIT: i64 = 96_000;
const PLAN_AUTO_COMPACT_TOKEN_LIMIT: i64 = 90_000;

fn built_in_profile(
    model: &str,
    model_provider: &str,
    model_reasoning_effort: Option<ReasoningEffort>,
    model_auto_compact_token_limit: i64,
) -> ConfigProfile {
    ConfigProfile {
        model: Some(model.to_string()),
        model_provider: Some(model_provider.to_string()),
        model_context_window: Some(CODING_CONTEXT_WINDOW),
        model_auto_compact_token_limit: Some(model_auto_compact_token_limit),
        model_reasoning_effort,
        ..Default::default()
    }
}

pub fn built_in_config_profiles() -> HashMap<String, ConfigProfile> {
    [
        (
            "qwen35-coding",
            built_in_profile(
                "qwen3.5-plus",
                "alibaba-coding",
                Some(ReasoningEffort::Medium),
                CODING_AUTO_COMPACT_TOKEN_LIMIT,
            ),
        ),
        (
            "qwen35-plan",
            built_in_profile(
                "qwen3.5-plus",
                "alibaba-coding",
                Some(ReasoningEffort::High),
                PLAN_AUTO_COMPACT_TOKEN_LIMIT,
            ),
        ),
        (
            "qwen3-coder-plus",
            built_in_profile(
                "qwen3-coder-plus",
                "alibaba-coding",
                Some(ReasoningEffort::Medium),
                CODING_AUTO_COMPACT_TOKEN_LIMIT,
            ),
        ),
        (
            "qwen3-coder-next",
            built_in_profile(
                "qwen3-coder-next",
                "alibaba-coding",
                Some(ReasoningEffort::Medium),
                CODING_AUTO_COMPACT_TOKEN_LIMIT,
            ),
        ),
        (
            "glm5-coding",
            built_in_profile(
                "glm-5",
                "alibaba-coding",
                Some(ReasoningEffort::Medium),
                CODING_AUTO_COMPACT_TOKEN_LIMIT,
            ),
        ),
        (
            "glm5-plan",
            built_in_profile(
                "glm-5",
                "alibaba-coding",
                Some(ReasoningEffort::High),
                PLAN_AUTO_COMPACT_TOKEN_LIMIT,
            ),
        ),
        (
            "glm5-zai-coding",
            built_in_profile(
                "glm-5",
                "zai-coding",
                Some(ReasoningEffort::Medium),
                CODING_AUTO_COMPACT_TOKEN_LIMIT,
            ),
        ),
        (
            "glm47-zai-coding",
            built_in_profile(
                "glm-4.7",
                "zai-coding",
                Some(ReasoningEffort::Medium),
                CODING_AUTO_COMPACT_TOKEN_LIMIT,
            ),
        ),
        (
            "openrouter-qwen",
            built_in_profile(
                "qwen/qwen3-coder-next",
                "openrouter",
                Some(ReasoningEffort::Medium),
                CODING_AUTO_COMPACT_TOKEN_LIMIT,
            ),
        ),
        (
            "deepseek-coding",
            built_in_profile(
                "deepseek-v3.2",
                "deepseek",
                Some(ReasoningEffort::Medium),
                CODING_AUTO_COMPACT_TOKEN_LIMIT,
            ),
        ),
    ]
    .into_iter()
    .map(|(name, profile)| (name.to_string(), profile))
    .collect()
}
