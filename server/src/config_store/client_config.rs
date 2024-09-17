use anyhow::anyhow;
use camino::Utf8Path;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ClientConfig {
    #[serde(skip)]
    pub machine_id: String,
    #[serde(flatten)]
    pub preflight: santa_types::Preflight,
    #[serde(default)]
    pub rules: Vec<santa_types::Rule>,
}

impl ClientConfig {
    pub fn from_file<P: AsRef<Utf8Path>>(path: P) -> Result<Self, anyhow::Error> {
        let path = path.as_ref();
        let contents = std::fs::read_to_string(path)?;
        let mut config: Self = match toml::from_str(&contents) {
            Ok(config) => config,
            Err(e) => return Err(anyhow!("could not parse TOML \"{}\", {}", path, e,)),
        };
        let machine_id = path.file_stem().map(|f| f.to_string()).unwrap();
        config.machine_id = machine_id;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_valid_config_file() {
        let config =
            ClientConfig::from_file("tests/tomls/client-tomls/good/machine-id-1234.toml").unwrap();
        assert_eq!("machine-id-1234".to_string(), config.machine_id);
        assert_eq!(600, config.preflight.full_sync_interval);
        assert_eq!(8, config.rules.len());
    }

    #[test]
    fn gracefully_handles_a_bad_config_file() {
        let result = ClientConfig::from_file("tests/tomls/client-tomls/bad/bad.toml");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("could not parse TOML"));
    }

    #[test]
    fn gracefully_handles_a_nonexistent_config_file() {
        let result = ClientConfig::from_file("tests/tomls/client-tomls/bad/nonexistent.toml");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.downcast_ref::<std::io::Error>().is_some());
    }
}
