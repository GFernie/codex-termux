use codex_protocol::config_types::Verbosity;
use codex_protocol::openai_models::ApplyPatchToolType;
use codex_protocol::openai_models::ConfigShellToolType;
use codex_protocol::openai_models::ModelInfo;
use codex_protocol::openai_models::ModelVisibility;
use codex_protocol::openai_models::ReasoningEffort;
use codex_protocol::openai_models::ReasoningEffortPreset;
use codex_protocol::openai_models::TruncationMode;
use codex_protocol::openai_models::TruncationPolicyConfig;

use crate::config::Config;
use crate::truncate::approx_bytes_for_tokens;
use tracing::warn;

pub const BASE_INSTRUCTIONS: &str = include_str!("../../prompt.md");
const BASE_INSTRUCTIONS_WITH_APPLY_PATCH: &str =
    include_str!("../../prompt_with_apply_patch_instructions.md");

const GPT_5_CODEX_INSTRUCTIONS: &str = include_str!("../../gpt_5_codex_prompt.md");
const GPT_5_1_INSTRUCTIONS: &str = include_str!("../../gpt_5_1_prompt.md");
const GPT_5_2_INSTRUCTIONS: &str = include_str!("../../gpt_5_2_prompt.md");
const GPT_5_1_CODEX_MAX_INSTRUCTIONS: &str = include_str!("../../gpt-5.1-codex-max_prompt.md");
const GPT_5_2_CODEX_INSTRUCTIONS: &str = include_str!("../../gpt-5.2-codex_prompt.md");

pub(crate) const CONTEXT_WINDOW_272K: i64 = 272_000;

macro_rules! model_info {
    (
        $slug:expr $(, $key:ident : $value:expr )* $(,)?
    ) => {{
        #[allow(unused_mut)]
        let mut model = ModelInfo {
            slug: $slug.to_string(),
            display_name: $slug.to_string(),
            description: None,
            // This is primarily used when remote metadata is available. When running
            // offline, core generally omits the effort field unless explicitly
            // configured by the user.
            default_reasoning_level: None,
            supported_reasoning_levels: supported_reasoning_level_low_medium_high(),
            shell_type: ConfigShellToolType::Default,
            visibility: ModelVisibility::None,
            supported_in_api: true,
            priority: 99,
            upgrade: None,
            base_instructions: BASE_INSTRUCTIONS.to_string(),
            supports_reasoning_summaries: false,
            support_verbosity: false,
            default_verbosity: None,
            apply_patch_tool_type: None,
            truncation_policy: TruncationPolicyConfig::bytes(10_000),
            supports_parallel_tool_calls: false,
            context_window: Some(CONTEXT_WINDOW_272K),
            auto_compact_token_limit: None,
            effective_context_window_percent: 95,
            experimental_supported_tools: Vec::new(),
        };

        $(
            model.$key = $value;
        )*
        model
    }};
}

pub(crate) fn with_config_overrides(mut model: ModelInfo, config: &Config) -> ModelInfo {
    if let Some(supports_reasoning_summaries) = config.model_supports_reasoning_summaries {
        model.supports_reasoning_summaries = supports_reasoning_summaries;
    }
    if let Some(context_window) = config.model_context_window {
        model.context_window = Some(context_window);
    }
    if let Some(auto_compact_token_limit) = config.model_auto_compact_token_limit {
        model.auto_compact_token_limit = Some(auto_compact_token_limit);
    }
    if let Some(token_limit) = config.tool_output_token_limit {
        model.truncation_policy = match model.truncation_policy.mode {
            TruncationMode::Bytes => {
                let byte_limit =
                    i64::try_from(approx_bytes_for_tokens(token_limit)).unwrap_or(i64::MAX);
                TruncationPolicyConfig::bytes(byte_limit)
            }
            TruncationMode::Tokens => {
                let limit = i64::try_from(token_limit).unwrap_or(i64::MAX);
                TruncationPolicyConfig::tokens(limit)
            }
        };
    }
    model
}

// todo(aibrahim): remove most of the entries here when enabling models.json
pub(crate) fn find_model_info_for_slug(slug: &str) -> ModelInfo {
    let slug_lower = slug.to_ascii_lowercase();
    let model_id = provider_agnostic_model_id(&slug_lower);

    if slug.starts_with("o3") || slug.starts_with("o4-mini") {
        model_info!(
            slug,
            base_instructions: BASE_INSTRUCTIONS_WITH_APPLY_PATCH.to_string(),
            supports_reasoning_summaries: true,
            context_window: Some(200_000),
        )
    } else if slug.starts_with("codex-mini-latest") {
        model_info!(
            slug,
            base_instructions: BASE_INSTRUCTIONS_WITH_APPLY_PATCH.to_string(),
            shell_type: ConfigShellToolType::Local,
            supports_reasoning_summaries: true,
            context_window: Some(200_000),
        )
    } else if slug.starts_with("gpt-4.1") {
        model_info!(
            slug,
            base_instructions: BASE_INSTRUCTIONS_WITH_APPLY_PATCH.to_string(),
            supports_reasoning_summaries: false,
            context_window: Some(1_047_576),
        )
    } else if slug.starts_with("gpt-oss") || slug.starts_with("openai/gpt-oss") {
        model_info!(
            slug,
            apply_patch_tool_type: Some(ApplyPatchToolType::Function),
            context_window: Some(96_000),
        )
    } else if slug.starts_with("gpt-4o") {
        model_info!(
            slug,
            base_instructions: BASE_INSTRUCTIONS_WITH_APPLY_PATCH.to_string(),
            supports_reasoning_summaries: false,
            context_window: Some(128_000),
        )
    } else if slug.starts_with("gpt-3.5") {
        model_info!(
            slug,
            base_instructions: BASE_INSTRUCTIONS_WITH_APPLY_PATCH.to_string(),
            supports_reasoning_summaries: false,
            context_window: Some(16_385),
        )
    } else if matches_any_prefix(
        model_id,
        &[
            "qwen3.5-plus",
            "qwen3.5",
            "qwen-plus",
            "qwen3-coder-plus",
            "qwen3-coder-next",
            "qwen3-coder-flash",
            "qwen3-coding",
            "qwen3-coding-next",
            "qwen2.5-coder",
            "qwen3-max",
            "qwen3-plus",
            "qwen3-thinking",
        ],
    ) || slug_lower.starts_with("zai-org/qwen")
    {
        model_info!(
            slug,
            apply_patch_tool_type: Some(ApplyPatchToolType::Function),
            context_window: Some(128_000),
        )
    } else if matches_any_prefix(
        model_id,
        &[
            "glm-5",
            "glm5",
            "glm-4.7",
            "glm4.7",
            "glm-4.6",
            "glm4.6",
            "glm-4.5",
            "glm4.5",
            "glm-4.5-air",
            "glm-4.5v",
            "glm-4.6v",
        ],
    ) || slug_lower.starts_with("zai-org/glm-")
    {
        model_info!(
            slug,
            apply_patch_tool_type: Some(ApplyPatchToolType::Function),
            context_window: Some(128_000),
        )
    } else if model_id.starts_with("deepseek") || slug_lower.starts_with("deepseek-ai/") {
        if model_id.contains("reasoner") || model_id.contains("thinking") {
            model_info!(
                slug,
                // Keep reasoning variants conservative for tool compatibility.
                apply_patch_tool_type: None,
                context_window: Some(128_000),
            )
        } else {
            model_info!(
                slug,
                apply_patch_tool_type: Some(ApplyPatchToolType::Function),
                context_window: Some(128_000),
            )
        }
    } else if model_id.starts_with("kimi-")
        || model_id.starts_with("moonshot-")
        || model_id.starts_with("moonshot-kimi-")
    {
        if model_id.contains("thinking") {
            model_info!(
                slug,
                apply_patch_tool_type: None,
                context_window: Some(128_000),
            )
        } else {
            model_info!(
                slug,
                apply_patch_tool_type: Some(ApplyPatchToolType::Function),
                context_window: Some(128_000),
            )
        }
    } else if slug.starts_with("test-gpt-5") {
        model_info!(
            slug,
            base_instructions: GPT_5_CODEX_INSTRUCTIONS.to_string(),
            experimental_supported_tools: vec![
                "grep_files".to_string(),
                "list_dir".to_string(),
                "read_file".to_string(),
                "test_sync_tool".to_string(),
            ],
            supports_parallel_tool_calls: true,
            supports_reasoning_summaries: true,
            shell_type: ConfigShellToolType::ShellCommand,
            support_verbosity: true,
            truncation_policy: TruncationPolicyConfig::tokens(10_000),
        )
    } else if slug.starts_with("exp-codex") || slug.starts_with("codex-1p") {
        model_info!(
            slug,
            base_instructions: GPT_5_2_CODEX_INSTRUCTIONS.to_string(),
            apply_patch_tool_type: Some(ApplyPatchToolType::Freeform),
            shell_type: ConfigShellToolType::ShellCommand,
            supports_parallel_tool_calls: true,
            supports_reasoning_summaries: true,
            support_verbosity: false,
            truncation_policy: TruncationPolicyConfig::tokens(10_000),
            context_window: Some(CONTEXT_WINDOW_272K),
        )
    } else if slug.starts_with("exp-") {
        model_info!(
            slug,
            apply_patch_tool_type: Some(ApplyPatchToolType::Freeform),
            supports_reasoning_summaries: true,
            support_verbosity: true,
            default_verbosity: Some(Verbosity::Low),
            base_instructions: BASE_INSTRUCTIONS.to_string(),
            default_reasoning_level: Some(ReasoningEffort::Medium),
            truncation_policy: TruncationPolicyConfig::bytes(10_000),
            shell_type: ConfigShellToolType::UnifiedExec,
            supports_parallel_tool_calls: true,
            context_window: Some(CONTEXT_WINDOW_272K),
        )
    } else if slug.starts_with("gpt-5.2-codex") || slug.starts_with("bengalfox") {
        model_info!(
            slug,
            base_instructions: GPT_5_2_CODEX_INSTRUCTIONS.to_string(),
            apply_patch_tool_type: Some(ApplyPatchToolType::Freeform),
            shell_type: ConfigShellToolType::ShellCommand,
            supports_parallel_tool_calls: true,
            supports_reasoning_summaries: true,
            support_verbosity: false,
            truncation_policy: TruncationPolicyConfig::tokens(10_000),
            context_window: Some(CONTEXT_WINDOW_272K),
            supported_reasoning_levels: supported_reasoning_level_low_medium_high_xhigh(),
        )
    } else if slug.starts_with("gpt-5.1-codex-max") {
        model_info!(
            slug,
            base_instructions: GPT_5_1_CODEX_MAX_INSTRUCTIONS.to_string(),
            apply_patch_tool_type: Some(ApplyPatchToolType::Freeform),
            shell_type: ConfigShellToolType::ShellCommand,
            supports_parallel_tool_calls: false,
            supports_reasoning_summaries: true,
            support_verbosity: false,
            truncation_policy: TruncationPolicyConfig::tokens(10_000),
            context_window: Some(CONTEXT_WINDOW_272K),
            supported_reasoning_levels: supported_reasoning_level_low_medium_high_xhigh(),
        )
    } else if (slug.starts_with("gpt-5-codex")
        || slug.starts_with("gpt-5.1-codex")
        || slug.starts_with("codex-"))
        && !slug.contains("-mini")
    {
        model_info!(
            slug,
            base_instructions: GPT_5_CODEX_INSTRUCTIONS.to_string(),
            apply_patch_tool_type: Some(ApplyPatchToolType::Freeform),
            shell_type: ConfigShellToolType::ShellCommand,
            supports_parallel_tool_calls: false,
            supports_reasoning_summaries: true,
            support_verbosity: false,
            truncation_policy: TruncationPolicyConfig::tokens(10_000),
            context_window: Some(CONTEXT_WINDOW_272K),
            supported_reasoning_levels: supported_reasoning_level_low_medium_high(),
        )
    } else if slug.starts_with("gpt-5-codex")
        || slug.starts_with("gpt-5.1-codex")
        || slug.starts_with("codex-")
    {
        model_info!(
            slug,
            base_instructions: GPT_5_CODEX_INSTRUCTIONS.to_string(),
            apply_patch_tool_type: Some(ApplyPatchToolType::Freeform),
            shell_type: ConfigShellToolType::ShellCommand,
            supports_parallel_tool_calls: false,
            supports_reasoning_summaries: true,
            support_verbosity: false,
            truncation_policy: TruncationPolicyConfig::tokens(10_000),
            context_window: Some(CONTEXT_WINDOW_272K),
        )
    } else if (slug.starts_with("gpt-5.2") || slug.starts_with("boomslang"))
        && !slug.contains("codex")
    {
        model_info!(
            slug,
            apply_patch_tool_type: Some(ApplyPatchToolType::Freeform),
            supports_reasoning_summaries: true,
            support_verbosity: true,
            default_verbosity: Some(Verbosity::Low),
            base_instructions: GPT_5_2_INSTRUCTIONS.to_string(),
            default_reasoning_level: Some(ReasoningEffort::Medium),
            truncation_policy: TruncationPolicyConfig::bytes(10_000),
            shell_type: ConfigShellToolType::ShellCommand,
            supports_parallel_tool_calls: true,
            context_window: Some(CONTEXT_WINDOW_272K),
            supported_reasoning_levels: supported_reasoning_level_low_medium_high_xhigh_non_codex(),
        )
    } else if slug.starts_with("gpt-5.1") && !slug.contains("codex") {
        model_info!(
            slug,
            apply_patch_tool_type: Some(ApplyPatchToolType::Freeform),
            supports_reasoning_summaries: true,
            support_verbosity: true,
            default_verbosity: Some(Verbosity::Low),
            base_instructions: GPT_5_1_INSTRUCTIONS.to_string(),
            default_reasoning_level: Some(ReasoningEffort::Medium),
            truncation_policy: TruncationPolicyConfig::bytes(10_000),
            shell_type: ConfigShellToolType::ShellCommand,
            supports_parallel_tool_calls: true,
            context_window: Some(CONTEXT_WINDOW_272K),
            supported_reasoning_levels: supported_reasoning_level_low_medium_high_non_codex(),
        )
    } else if slug.starts_with("gpt-5") {
        model_info!(
            slug,
            base_instructions: BASE_INSTRUCTIONS_WITH_APPLY_PATCH.to_string(),
            shell_type: ConfigShellToolType::Default,
            supports_reasoning_summaries: true,
            support_verbosity: true,
            truncation_policy: TruncationPolicyConfig::bytes(10_000),
            context_window: Some(CONTEXT_WINDOW_272K),
        )
    } else {
        warn!("Unknown model {slug} is used. This will degrade the performance of Codex.");
        model_info!(
            slug,
            context_window: None,
            supported_reasoning_levels: Vec::new(),
            default_reasoning_level: None
        )
    }
}

fn provider_agnostic_model_id(slug_lower: &str) -> &str {
    slug_lower.rsplit('/').next().unwrap_or(slug_lower)
}

fn matches_any_prefix(value: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|prefix| value.starts_with(prefix))
}

fn supported_reasoning_level_low_medium_high() -> Vec<ReasoningEffortPreset> {
    vec![
        ReasoningEffortPreset {
            effort: ReasoningEffort::Low,
            description: "Fast responses with lighter reasoning".to_string(),
        },
        ReasoningEffortPreset {
            effort: ReasoningEffort::Medium,
            description: "Balances speed and reasoning depth for everyday tasks".to_string(),
        },
        ReasoningEffortPreset {
            effort: ReasoningEffort::High,
            description: "Greater reasoning depth for complex problems".to_string(),
        },
    ]
}

fn supported_reasoning_level_low_medium_high_non_codex() -> Vec<ReasoningEffortPreset> {
    vec![
        ReasoningEffortPreset {
            effort: ReasoningEffort::Low,
            description: "Balances speed with some reasoning; useful for straightforward queries and short explanations".to_string(),
        },
        ReasoningEffortPreset {
            effort: ReasoningEffort::Medium,
            description: "Provides a solid balance of reasoning depth and latency for general-purpose tasks".to_string(),
        },
        ReasoningEffortPreset {
            effort: ReasoningEffort::High,
            description: "Maximizes reasoning depth for complex or ambiguous problems".to_string(),
        },
    ]
}

fn supported_reasoning_level_low_medium_high_xhigh() -> Vec<ReasoningEffortPreset> {
    vec![
        ReasoningEffortPreset {
            effort: ReasoningEffort::Low,
            description: "Fast responses with lighter reasoning".to_string(),
        },
        ReasoningEffortPreset {
            effort: ReasoningEffort::Medium,
            description: "Balances speed and reasoning depth for everyday tasks".to_string(),
        },
        ReasoningEffortPreset {
            effort: ReasoningEffort::High,
            description: "Greater reasoning depth for complex problems".to_string(),
        },
        ReasoningEffortPreset {
            effort: ReasoningEffort::XHigh,
            description: "Extra high reasoning depth for complex problems".to_string(),
        },
    ]
}

fn supported_reasoning_level_low_medium_high_xhigh_non_codex() -> Vec<ReasoningEffortPreset> {
    vec![
        ReasoningEffortPreset {
            effort: ReasoningEffort::Low,
            description: "Balances speed with some reasoning; useful for straightforward queries and short explanations".to_string(),
        },
        ReasoningEffortPreset {
            effort: ReasoningEffort::Medium,
            description: "Provides a solid balance of reasoning depth and latency for general-purpose tasks".to_string(),
        },
        ReasoningEffortPreset {
            effort: ReasoningEffort::High,
            description: "Maximizes reasoning depth for complex or ambiguous problems".to_string(),
        },
        ReasoningEffortPreset {
            effort: ReasoningEffort::XHigh,
            description: "Extra high reasoning for complex problems".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qwen35_plus_has_known_model_profile() {
        let info = find_model_info_for_slug("qwen3.5-plus");
        assert_eq!(
            info.apply_patch_tool_type,
            Some(ApplyPatchToolType::Function)
        );
        assert_eq!(info.context_window, Some(128_000));
    }

    #[test]
    fn glm5_has_known_model_profile_case_insensitive() {
        let info = find_model_info_for_slug("GLM-5");
        assert_eq!(
            info.apply_patch_tool_type,
            Some(ApplyPatchToolType::Function)
        );
        assert_eq!(info.context_window, Some(128_000));
    }

    #[test]
    fn qwen3_coder_plus_has_known_model_profile() {
        let info = find_model_info_for_slug("qwen3-coder-plus");
        assert_eq!(
            info.apply_patch_tool_type,
            Some(ApplyPatchToolType::Function)
        );
        assert_eq!(info.context_window, Some(128_000));
    }

    #[test]
    fn qwen3_coding_next_has_known_model_profile() {
        let info = find_model_info_for_slug("qwen3-coding-next");
        assert_eq!(
            info.apply_patch_tool_type,
            Some(ApplyPatchToolType::Function)
        );
        assert_eq!(info.context_window, Some(128_000));
    }

    #[test]
    fn qwen3_coder_next_openrouter_style_slug_is_supported() {
        let info = find_model_info_for_slug("qwen/qwen3-coder-next");
        assert_eq!(
            info.apply_patch_tool_type,
            Some(ApplyPatchToolType::Function)
        );
        assert_eq!(info.context_window, Some(128_000));
    }

    #[test]
    fn deepseek_v32_has_known_model_profile() {
        let info = find_model_info_for_slug("deepseek-v3.2");
        assert_eq!(
            info.apply_patch_tool_type,
            Some(ApplyPatchToolType::Function)
        );
        assert_eq!(info.context_window, Some(128_000));
    }

    #[test]
    fn deepseek_reasoner_uses_conservative_profile() {
        let info = find_model_info_for_slug("deepseek-reasoner");
        assert_eq!(info.apply_patch_tool_type, None);
        assert_eq!(info.context_window, Some(128_000));
    }

    #[test]
    fn deepseek_reasoner_namespaced_slug_uses_conservative_profile() {
        let info = find_model_info_for_slug("deepseek/deepseek-reasoner");
        assert_eq!(info.apply_patch_tool_type, None);
        assert_eq!(info.context_window, Some(128_000));
    }

    #[test]
    fn kimi_k25_has_known_model_profile() {
        let info = find_model_info_for_slug("kimi-k2.5");
        assert_eq!(
            info.apply_patch_tool_type,
            Some(ApplyPatchToolType::Function)
        );
        assert_eq!(info.context_window, Some(128_000));
    }

    #[test]
    fn kimi_namespaced_slug_has_known_model_profile() {
        let info = find_model_info_for_slug("moonshotai/kimi-k2.5");
        assert_eq!(
            info.apply_patch_tool_type,
            Some(ApplyPatchToolType::Function)
        );
        assert_eq!(info.context_window, Some(128_000));
    }

    #[test]
    fn kimi_thinking_uses_conservative_profile() {
        let info = find_model_info_for_slug("kimi-thinking-preview");
        assert_eq!(info.apply_patch_tool_type, None);
        assert_eq!(info.context_window, Some(128_000));
    }

    #[test]
    fn zai_org_glm5_has_known_model_profile() {
        let info = find_model_info_for_slug("zai-org/GLM-5-TEE");
        assert_eq!(
            info.apply_patch_tool_type,
            Some(ApplyPatchToolType::Function)
        );
        assert_eq!(info.context_window, Some(128_000));
    }

    #[test]
    fn glm47_has_known_model_profile() {
        let info = find_model_info_for_slug("GLM-4.7");
        assert_eq!(
            info.apply_patch_tool_type,
            Some(ApplyPatchToolType::Function)
        );
        assert_eq!(info.context_window, Some(128_000));
    }
}
