use std::str::FromStr;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use crate::models::Command;

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