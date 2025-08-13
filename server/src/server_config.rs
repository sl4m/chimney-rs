use std::env;
use std::net::SocketAddr;

use anyhow::anyhow;
use camino::{Utf8Path, Utf8PathBuf};
use dropshot::{ConfigLoggingIfExists, ConfigLoggingLevel};
use serde::{Deserialize, Serialize};

const ENV_VAR_CONFIG: &str = "CHIMNEY_CONFIG";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub bind_address: SocketAddr,
    pub client_config_path: Utf8PathBuf,
    pub event_log_path: Option<Utf8PathBuf>,
    #[serde(default = "log_level_default")]
    pub log_level: ConfigLoggingLevel,
    #[serde(default = "log_mode")]
    pub log_mode: ConfigLoggingIfExists,
    pub log_path: Utf8PathBuf,
    pub tls_config: Option<TlsConfig>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum EventLogMode {
    #[serde(rename = "combined")]
    Combined,
    // TODO: need to implement
    //#[serde(rename = "per_executable")]
    //PerExecutable,
}

impl ServerConfig {
    pub fn from_file<P: AsRef<Utf8Path>>(maybe_path: Option<P>) -> Result<Self, anyhow::Error> {
        let path = maybe_path
            .map(|p| p.as_ref().to_owned())
            .or_else(|| env::var(ENV_VAR_CONFIG).ok().map(Utf8PathBuf::from))
            .ok_or_else(|| anyhow!("chimney server config path not defined"))?;
        let contents = std::fs::read_to_string(&path)?;
        let config: Self = match toml::from_str(&contents) {
            Ok(config) => config,
            Err(e) => return Err(anyhow!("could not parse TOML \"{}\", {}", path, e,)),
        };
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), anyhow::Error> {
        if self.log_path.is_dir() {
            return Err(anyhow!(
                "log path \"{}\" must include the log filename",
                self.log_path
            ));
        }
        if let Some(path) = &self.event_log_path
            && path.is_dir()
        {
            return Err(anyhow!(
                "event log path \"{}\" must include the log filename",
                path
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TlsConfig {
    pub cert_file: Utf8PathBuf,
    pub key_file: Utf8PathBuf,
}

fn log_level_default() -> ConfigLoggingLevel {
    ConfigLoggingLevel::Info
}

fn log_mode() -> ConfigLoggingIfExists {
    ConfigLoggingIfExists::Append
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ffi::OsStr;
    use std::hash::Hash;
    use std::panic::{self, RefUnwindSafe, UnwindSafe};
    use std::sync::{LazyLock, Mutex};

    use super::*;

    const GOOD_CONFIG: &str = "tests/tomls/server-tomls/good/config.toml";

    // The following stanzas of code should be attributed to https://github.com/containers/netavark

    static SERIAL_TEST: LazyLock<Mutex<()>> = LazyLock::new(Default::default);

    /// The following stanzas of code should be attributed to https://github.com/vmx/temp-env
    ///
    /// The previous value is restored when the closure completes or panics, before unwinding the
    /// panic.
    ///
    /// If `value` is set to `None`, then the environment variable is unset.
    pub fn with_var<K, V, F, R>(key: K, value: Option<V>, closure: F) -> R
    where
        K: AsRef<OsStr> + Clone + Eq + Hash,
        V: AsRef<OsStr> + Clone,
        F: Fn() -> R + UnwindSafe + RefUnwindSafe,
    {
        with_vars(vec![(key, value)], closure)
    }

    /// Unsets a single environment variable for the duration of the closure.
    ///
    /// The previous value is restored when the closure completes or panics, before unwinding the
    /// panic.
    ///
    /// This is a shorthand and identical to the following:
    /// ```rust
    /// temp_env::with_var("MY_ENV_VAR", None::<&str>, || {
    ///     // Run some code where `MY_ENV_VAR` is unset.
    /// });
    /// ```
    pub fn with_var_unset<K, F, R>(key: K, closure: F) -> R
    where
        K: AsRef<OsStr> + Clone + Eq + Hash,
        F: Fn() -> R + UnwindSafe + RefUnwindSafe,
    {
        with_var(key, None::<&str>, closure)
    }

    /// Sets environment variables for the duration of the closure.
    ///
    /// The previous values are restored when the closure completes or panics, before unwinding the
    /// panic.
    ///
    /// If a `value` is set to `None`, then the environment variable is unset.
    ///
    /// If the variable with the same name is set multiple times, the last one wins.
    pub fn with_vars<K, V, F, R>(kvs: Vec<(K, Option<V>)>, closure: F) -> R
    where
        K: AsRef<OsStr> + Clone + Eq + Hash,
        V: AsRef<OsStr> + Clone,
        F: Fn() -> R + UnwindSafe + RefUnwindSafe,
    {
        let guard = SERIAL_TEST.lock().unwrap();
        let mut old_kvs: HashMap<K, Option<String>> = HashMap::new();
        for (key, value) in kvs {
            // If the same key is given several times, the original/old value is only correct before
            // the environment was updated.
            if !old_kvs.contains_key(&key) {
                let old_value = env::var(&key).ok();
                old_kvs.insert(key.clone(), old_value);
            }
            update_env(&key, value);
        }

        match panic::catch_unwind(closure) {
            Ok(result) => {
                for (key, value) in old_kvs {
                    update_env(key, value);
                }
                result
            }
            Err(err) => {
                for (key, value) in old_kvs {
                    update_env(key, value);
                }
                drop(guard);
                panic::resume_unwind(err);
            }
        }
    }

    fn update_env<K, V>(key: K, value: Option<V>)
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        match value {
            Some(v) => unsafe { env::set_var(key, v) },
            None => unsafe { env::remove_var(key) },
        }
    }

    #[cfg(not(windows))]
    #[test]
    fn parses_valid_config_file() {
        let path = Some(GOOD_CONFIG);
        let config = ServerConfig::from_file(path).unwrap();
        assert_eq!(log_level_default(), config.log_level);
        assert_eq!(log_mode(), config.log_mode);
        assert!(config.event_log_path.is_none());
        assert!(config.tls_config.is_none());
    }

    #[cfg(not(windows))]
    #[test]
    fn parses_valid_config_file_with_tls() {
        let path = Some("tests/tomls/server-tomls/good/config_tls.toml");
        let config = ServerConfig::from_file(path).unwrap();
        assert!(config.tls_config.is_some());
    }

    #[cfg(not(windows))]
    #[test]
    fn sets_event_log_path() {
        let path = Some("tests/tomls/server-tomls/good/config_event_log_path.toml");
        let config = ServerConfig::from_file(path).unwrap();
        assert!(config.event_log_path.is_some());
        assert_eq!(
            "/tmp/log/chimney-events.log",
            config.event_log_path.unwrap()
        );
    }

    #[cfg(not(windows))]
    #[test]
    fn sets_log_level() {
        let path = Some("tests/tomls/server-tomls/good/config_log_level.toml");
        let config = ServerConfig::from_file(path).unwrap();
        assert_eq!(dropshot::ConfigLoggingLevel::Debug, config.log_level);
    }

    #[test]
    fn errs_on_invalid_config_file() {
        let path = Some("tests/tomls/server-tomls/bad/config.toml");
        let result = ServerConfig::from_file(path);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("could not parse TOML"));
    }

    #[test]
    fn errs_on_nonexistent_config_file() {
        let path = Some("tests/tomls/server-tomls/bad/nonexistent.toml");
        let result = ServerConfig::from_file(path);
        assert!(result.is_err());
    }

    #[cfg(not(windows))]
    #[test]
    fn falls_back_on_env_var() {
        with_var(ENV_VAR_CONFIG, Some(GOOD_CONFIG), || {
            let config = ServerConfig::from_file::<Utf8PathBuf>(None).unwrap();
            assert_eq!(log_level_default(), config.log_level);
            assert_eq!(log_mode(), config.log_mode);
            assert!(config.event_log_path.is_none());
            assert!(config.tls_config.is_none());
        });
    }

    #[cfg(not(windows))]
    #[test]
    fn errs_on_missing_config_file_path() {
        with_var_unset(ENV_VAR_CONFIG, || {
            let result = ServerConfig::from_file::<Utf8PathBuf>(None);
            assert!(result.is_err());
        });
    }

    #[cfg(not(windows))]
    #[test]
    fn errs_on_invalid_log_path() {
        let path = Some("tests/tomls/server-tomls/bad/log_path.toml");
        let result = ServerConfig::from_file(path);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert_eq!("log path \"/tmp\" must include the log filename", err_msg);
    }

    #[cfg(not(windows))]
    #[test]
    fn errs_on_invalid_event_log_path() {
        let path = Some("tests/tomls/server-tomls/bad/event_log_path.toml");
        let result = ServerConfig::from_file(path);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert_eq!(
            "event log path \"/tmp\" must include the log filename",
            err_msg
        );
    }
}
