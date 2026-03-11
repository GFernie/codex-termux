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
const ALIBABA_QWEN_CONTEXT_WINDOW: i64 = 1_000_000;
const ALIBABA_QWEN_CODER_NEXT_CONTEXT_WINDOW: i64 = 262_144;
const ALIBABA_QWEN3_MAX_CONTEXT_WINDOW: i64 = 262_144;
const ALIBABA_GLM5_CONTEXT_WINDOW: i64 = 202_752;
const ALIBABA_GLM47_CONTEXT_WINDOW: i64 = 202_752;
const ALIBABA_KIMI_K25_CONTEXT_WINDOW: i64 = 262_144;
const ALIBABA_MINIMAX_M25_CONTEXT_WINDOW: i64 = 204_800;
const ZAI_GLM5_CONTEXT_WINDOW: i64 = 200_000;
const ZAI_GLM47_CONTEXT_WINDOW: i64 = 200_000;

fn built_in_profile(
    model: &str,
    model_provider: &str,
    model_reasoning_effort: Option<ReasoningEffort>,
) -> ConfigProfile {
    let context_window = built_in_profile_context_window(model_provider, model);
    ConfigProfile {
        model: Some(model.to_string()),
        model_provider: Some(model_provider.to_string()),
        model_context_window: Some(context_window),
        model_auto_compact_token_limit: Some(exact_auto_compact_token_limit(context_window)),
        model_reasoning_effort,
        ..Default::default()
    }
}

fn built_in_profile_context_window(model_provider: &str, model: &str) -> i64 {
    match (model_provider, model.to_ascii_lowercase().as_str()) {
        ("alibaba-coding", "qwen3.5-plus") => ALIBABA_QWEN_CONTEXT_WINDOW,
        ("alibaba-coding", "qwen3-coder-plus") => ALIBABA_QWEN_CONTEXT_WINDOW,
        ("alibaba-coding", "qwen3-coder-next") => ALIBABA_QWEN_CODER_NEXT_CONTEXT_WINDOW,
        ("alibaba-coding", "qwen3-max-2026-01-23") => ALIBABA_QWEN3_MAX_CONTEXT_WINDOW,
        ("alibaba-coding", "glm-5") => ALIBABA_GLM5_CONTEXT_WINDOW,
        ("alibaba-coding", "glm-4.7") => ALIBABA_GLM47_CONTEXT_WINDOW,
        ("alibaba-coding", "kimi-k2.5") => ALIBABA_KIMI_K25_CONTEXT_WINDOW,
        ("alibaba-coding", "minimax-m2.5") => ALIBABA_MINIMAX_M25_CONTEXT_WINDOW,
        ("zai-coding", "glm-5") => ZAI_GLM5_CONTEXT_WINDOW,
        ("zai-coding", "glm-4.7") => ZAI_GLM47_CONTEXT_WINDOW,
        _ => CODING_CONTEXT_WINDOW,
    }
}

fn exact_auto_compact_token_limit(context_window: i64) -> i64 {
    context_window.saturating_mul(3) / 4
}

pub fn built_in_config_profiles() -> HashMap<String, ConfigProfile> {
    [
        (
            "qwen35-coding",
            built_in_profile(
                "qwen3.5-plus",
                "alibaba-coding",
                Some(ReasoningEffort::Medium),
            ),
        ),
        (
            "qwen35-plan",
            built_in_profile(
                "qwen3.5-plus",
                "alibaba-coding",
                Some(ReasoningEffort::High),
            ),
        ),
        (
            "qwen3-coder-plus",
            built_in_profile(
                "qwen3-coder-plus",
                "alibaba-coding",
                Some(ReasoningEffort::Medium),
            ),
        ),
        (
            "qwen3-coder-next",
            built_in_profile(
                "qwen3-coder-next",
                "alibaba-coding",
                Some(ReasoningEffort::Medium),
            ),
        ),
        (
            "qwen3max-plan",
            built_in_profile(
                "qwen3-max-2026-01-23",
                "alibaba-coding",
                Some(ReasoningEffort::High),
            ),
        ),
        (
            "glm5-coding",
            built_in_profile("glm-5", "alibaba-coding", Some(ReasoningEffort::Medium)),
        ),
        (
            "glm5-plan",
            built_in_profile("glm-5", "alibaba-coding", Some(ReasoningEffort::High)),
        ),
        (
            "glm47-coding",
            built_in_profile("glm-4.7", "alibaba-coding", Some(ReasoningEffort::Medium)),
        ),
        (
            "kimi-k25-coding",
            built_in_profile("kimi-k2.5", "alibaba-coding", Some(ReasoningEffort::Medium)),
        ),
        (
            "minimax-m25-coding",
            built_in_profile(
                "minimax-m2.5",
                "alibaba-coding",
                Some(ReasoningEffort::Medium),
            ),
        ),
        (
            "glm5-zai-coding",
            built_in_profile("glm-5", "zai-coding", Some(ReasoningEffort::Medium)),
        ),
        (
            "glm47-zai-coding",
            built_in_profile("glm-4.7", "zai-coding", Some(ReasoningEffort::Medium)),
        ),
        (
            "openrouter-qwen",
            built_in_profile(
                "qwen/qwen3-coder-next",
                "openrouter",
                Some(ReasoningEffort::Medium),
            ),
        ),
        (
            "deepseek-coding",
            built_in_profile("deepseek-v3.2", "deepseek", Some(ReasoningEffort::Medium)),
        ),
    ]
    .into_iter()
    .map(|(name, profile)| (name.to_string(), profile))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alibaba_qwen35_profile_uses_exact_context() {
        let profile = built_in_config_profiles()
            .remove("qwen35-coding")
            .expect("qwen35-coding profile");
        assert_eq!(profile.model_context_window, Some(1_000_000));
        assert_eq!(profile.model_auto_compact_token_limit, Some(750_000));
    }

    #[test]
    fn alibaba_glm47_profile_is_available() {
        let profile = built_in_config_profiles()
            .remove("glm47-coding")
            .expect("glm47-coding profile");
        assert_eq!(profile.model.as_deref(), Some("glm-4.7"));
        assert_eq!(profile.model_provider.as_deref(), Some("alibaba-coding"));
        assert_eq!(profile.model_context_window, Some(202_752));
        assert_eq!(profile.model_auto_compact_token_limit, Some(152_064));
    }
}
