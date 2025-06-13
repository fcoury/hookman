use crate::models::Command;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum, Default)]
#[serde(rename_all = "kebab-case")]
pub enum HookType {
    #[default]
    PreCommit,
    PrePush,
    CommitMsg,
    PostCommit,
    PreRebase,
    PostCheckout,
    PostMerge,
    PreReceive,
    Update,
    PostReceive,
    PrepareCommitMsg,
    PostUpdate,
    PreApplyPatch,
    PostApplyPatch,
    PreMerge,
    PostRewrite,
}

impl HookType {
    pub fn as_str(&self) -> &'static str {
        match self {
            HookType::PreCommit => "pre-commit",
            HookType::PrePush => "pre-push",
            HookType::CommitMsg => "commit-msg",
            HookType::PostCommit => "post-commit",
            HookType::PreRebase => "pre-rebase",
            HookType::PostCheckout => "post-checkout",
            HookType::PostMerge => "post-merge",
            HookType::PreReceive => "pre-receive",
            HookType::Update => "update",
            HookType::PostReceive => "post-receive",
            HookType::PrepareCommitMsg => "prepare-commit-msg",
            HookType::PostUpdate => "post-update",
            HookType::PreApplyPatch => "pre-applypatch",
            HookType::PostApplyPatch => "post-applypatch",
            HookType::PreMerge => "pre-merge-commit",
            HookType::PostRewrite => "post-rewrite",
        }
    }

    pub fn all() -> Vec<HookType> {
        vec![
            HookType::PreCommit,
            HookType::PrePush,
            HookType::CommitMsg,
            HookType::PostCommit,
            HookType::PreRebase,
            HookType::PostCheckout,
            HookType::PostMerge,
            HookType::PreReceive,
            HookType::Update,
            HookType::PostReceive,
            HookType::PrepareCommitMsg,
            HookType::PostUpdate,
            HookType::PreApplyPatch,
            HookType::PostApplyPatch,
            HookType::PreMerge,
            HookType::PostRewrite,
        ]
    }
}

impl std::fmt::Display for HookType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for HookType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pre-commit" => Ok(HookType::PreCommit),
            "pre-push" => Ok(HookType::PrePush),
            "commit-msg" => Ok(HookType::CommitMsg),
            "post-commit" => Ok(HookType::PostCommit),
            "pre-rebase" => Ok(HookType::PreRebase),
            "post-checkout" => Ok(HookType::PostCheckout),
            "post-merge" => Ok(HookType::PostMerge),
            "pre-receive" => Ok(HookType::PreReceive),
            "update" => Ok(HookType::Update),
            "post-receive" => Ok(HookType::PostReceive),
            "prepare-commit-msg" => Ok(HookType::PrepareCommitMsg),
            "post-update" => Ok(HookType::PostUpdate),
            "pre-applypatch" => Ok(HookType::PreApplyPatch),
            "post-applypatch" => Ok(HookType::PostApplyPatch),
            "pre-merge-commit" => Ok(HookType::PreMerge),
            "post-rewrite" => Ok(HookType::PostRewrite),
            _ => Err(format!("Unknown hook type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hook {
    #[serde(skip_deserializing)]
    pub hook_type: HookType,
    pub commands: Vec<Command>,
}

impl Hook {
    pub fn new(hook_type: HookType) -> Self {
        Hook {
            hook_type,
            commands: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_type_as_str() {
        assert_eq!(HookType::PreCommit.as_str(), "pre-commit");
        assert_eq!(HookType::PrePush.as_str(), "pre-push");
        assert_eq!(HookType::CommitMsg.as_str(), "commit-msg");
    }

    #[test]
    fn test_hook_type_from_str() {
        use std::str::FromStr;
        assert_eq!(
            <HookType as FromStr>::from_str("pre-commit").unwrap(),
            HookType::PreCommit
        );
        assert_eq!(
            <HookType as FromStr>::from_str("pre-push").unwrap(),
            HookType::PrePush
        );
        assert_eq!(
            <HookType as FromStr>::from_str("commit-msg").unwrap(),
            HookType::CommitMsg
        );
        assert!(<HookType as FromStr>::from_str("invalid-hook").is_err());
    }

    #[test]
    fn test_hook_type_display() {
        assert_eq!(format!("{}", HookType::PreCommit), "pre-commit");
        assert_eq!(format!("{}", HookType::PrePush), "pre-push");
    }

    #[test]
    fn test_hook_type_all() {
        let all_hooks = HookType::all();
        assert_eq!(all_hooks.len(), 16);
        assert!(all_hooks.contains(&HookType::PreCommit));
        assert!(all_hooks.contains(&HookType::PostRewrite));
    }

    #[test]
    fn test_hook_new() {
        let hook = Hook::new(HookType::PreCommit);
        assert_eq!(hook.hook_type, HookType::PreCommit);
        assert!(hook.commands.is_empty());
    }
}
