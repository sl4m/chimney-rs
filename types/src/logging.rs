use slog::{KV, Record};

use crate::Event;
use crate::event_upload::Decision;

impl KV for Event {
    fn serialize(&self, record: &Record, serializer: &mut dyn slog::Serializer) -> slog::Result {
        serializer.emit_str("file_sha256".into(), &self.file_sha256)?;
        serializer.emit_str("file_path".into(), &self.file_path)?;
        serializer.emit_str("file_name".into(), &self.file_name)?;
        if let Some(executing_user) = &self.executing_user.as_ref() {
            serializer.emit_str("executing_user".into(), executing_user)?;
        }
        if let Some(execution_time) = self.execution_time {
            serializer.emit_f64("execution_time".into(), execution_time)?;
        }
        let loggedin_users: Vec<String> =
            self.loggedin_users.iter().map(|s| s.to_string()).collect();
        serializer.emit_str("loggedin_users".into(), &loggedin_users.join(", "))?;
        let current_sessions: Vec<String> = self
            .current_sessions
            .iter()
            .map(|s| s.to_string())
            .collect();
        serializer.emit_str("current_sessions".into(), &current_sessions.join(", "))?;
        KV::serialize(&self.decision, record, serializer)?;
        if let Some(file_bundle_id) = &self.file_bundle_id {
            serializer.emit_str("file_bundle_id".into(), file_bundle_id)?;
        }
        if let Some(file_bundle_path) = &self.file_bundle_path {
            serializer.emit_str("file_bundle_path".into(), file_bundle_path)?;
        }
        if let Some(file_bundle_executable_rel_path) = &self.file_bundle_executable_rel_path {
            serializer.emit_str(
                "file_bundle_executable_rel_path".into(),
                file_bundle_executable_rel_path,
            )?;
        }
        if let Some(file_bundle_name) = &self.file_bundle_name {
            serializer.emit_str("file_bundle_name".into(), file_bundle_name)?;
        }
        if let Some(file_bundle_version) = &self.file_bundle_version {
            serializer.emit_str("file_bundle_version".into(), file_bundle_version)?;
        }
        if let Some(file_bundle_version_string) = &self.file_bundle_version_string {
            serializer.emit_str(
                "file_bundle_version_string".into(),
                file_bundle_version_string,
            )?;
        }
        if let Some(file_bundle_hash) = &self.file_bundle_hash {
            serializer.emit_str("file_bundle_hash".into(), file_bundle_hash)?;
        }
        if let Some(file_bundle_hash_millis) = self.file_bundle_hash_millis {
            serializer.emit_u32("file_bundle_hash_millis".into(), file_bundle_hash_millis)?;
        }
        if let Some(file_bundle_binary_count) = self.file_bundle_binary_count {
            serializer.emit_u32("file_bundle_binary_count".into(), file_bundle_binary_count)?;
        }
        if let Some(pid) = self.pid {
            serializer.emit_u32("pid".into(), pid)?;
        }
        if let Some(ppid) = self.ppid {
            serializer.emit_u32("ppid".into(), ppid)?;
        }
        if let Some(parent_name) = &self.parent_name {
            serializer.emit_str("parent_name".into(), parent_name)?;
        }
        if let Some(quarantine_data_url) = &self.quarantine_data_url {
            serializer.emit_str("quarantine_data_url".into(), quarantine_data_url)?;
        }
        if let Some(quarantine_referer_url) = &self.quarantine_referer_url {
            serializer.emit_str("quarantine_referer_url".into(), quarantine_referer_url)?;
        }
        if let Some(quarantine_timestamp) = self.quarantine_timestamp {
            serializer.emit_f64("quarantine_timestamp".into(), quarantine_timestamp)?;
        }
        if let Some(quarantine_agent_bundle_id) = &self.quarantine_agent_bundle_id {
            serializer.emit_str(
                "quarantine_agent_bundle_id".into(),
                quarantine_agent_bundle_id,
            )?;
        }
        for (index, signing_chain) in self.signing_chain.iter().enumerate() {
            serializer.emit_str(
                format!("signing_chain.{index}.sha256").into(),
                &signing_chain.sha256,
            )?;
            serializer.emit_str(
                format!("signing_chain.{index}.cn").into(),
                &signing_chain.cn,
            )?;
            serializer.emit_str(
                format!("signing_chain.{index}.org").into(),
                &signing_chain.org,
            )?;
            serializer.emit_str(
                format!("signing_chain.{index}.ou").into(),
                &signing_chain.ou,
            )?;
            serializer.emit_u32(
                format!("signing_chain.{index}.valid_from").into(),
                signing_chain.valid_from,
            )?;
            serializer.emit_u32(
                format!("signing_chain.{index}.valid_until").into(),
                signing_chain.valid_until,
            )?;
        }
        if let Some(signing_id) = &self.signing_id {
            serializer.emit_str("signing_id".into(), signing_id)?;
        }
        if let Some(team_id) = &self.team_id {
            serializer.emit_str("team_id".into(), team_id)?;
        }
        if let Some(cdhash) = &self.cdhash {
            serializer.emit_str("cdhash".into(), cdhash)?;
        }

        Ok(())
    }
}

impl KV for Decision {
    fn serialize(&self, _record: &Record, serializer: &mut dyn slog::Serializer) -> slog::Result {
        let key = "decision".into();
        match self {
            Decision::AllowBinary => serializer.emit_str(key, "ALLOW_BINARY"),
            Decision::AllowCertificate => serializer.emit_str(key, "ALLOW_CERTIFICATE"),
            Decision::AllowCdHash => serializer.emit_str(key, "ALLOW_CDHASH"),
            Decision::AllowScope => serializer.emit_str(key, "ALLOW_SCOPE"),
            Decision::AllowSigningId => serializer.emit_str(key, "ALLOW_SIGNINGID"),
            Decision::AllowTeamId => serializer.emit_str(key, "ALLOW_TEAMID"),
            Decision::AllowUnknown => serializer.emit_str(key, "ALLOW_UNKNOWN"),
            Decision::BlockBinary => serializer.emit_str(key, "BLOCK_BINARY"),
            Decision::BlockCertificate => serializer.emit_str(key, "BLOCK_CERTIFICATE"),
            Decision::BlockCdHash => serializer.emit_str(key, "BLOCK_CDHASH"),
            Decision::BlockScope => serializer.emit_str(key, "BLOCK_SCOPE"),
            Decision::BlockSigningId => serializer.emit_str(key, "BLOCK_SIGNINGID"),
            Decision::BlockTeamId => serializer.emit_str(key, "BLOCK_TEAMID"),
            Decision::BlockUnknown => serializer.emit_str(key, "BLOCK_UNKNOWN"),
            Decision::BundleBinary => serializer.emit_str(key, "BUNDLE_BINARY"),
        }
    }
}
