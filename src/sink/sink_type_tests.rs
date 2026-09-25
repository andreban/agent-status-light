use std::error::Error as StdError;
use std::str::FromStr;

use super::*;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_sink_type_as_str_and_display() -> Result<()> {
    assert_eq!(SinkType::Serial.as_str(), "serial");
    assert_eq!(SinkType::Http.as_str(), "http");
    assert_eq!(SinkType::Stdout.as_str(), "stdout");

    assert_eq!(SinkType::Serial.to_string(), "serial");
    assert_eq!(SinkType::Http.to_string(), "http");
    assert_eq!(SinkType::Stdout.to_string(), "stdout");

    assert_eq!(SinkType::Serial.as_ref(), "serial");

    let s: &'static str = SinkType::Serial.into();
    assert_eq!(s, "serial");

    assert_eq!(SinkType::default(), SinkType::Serial);
    Ok(())
}

#[test]
fn test_sink_type_from_str() -> Result<()> {
    assert_eq!(SinkType::from_str("serial")?, SinkType::Serial);
    assert_eq!(SinkType::from_str("http")?, SinkType::Http);
    assert_eq!(SinkType::from_str("stdout")?, SinkType::Stdout);

    assert_eq!(SinkType::try_from("serial")?, SinkType::Serial);
    assert_eq!(SinkType::try_from("http")?, SinkType::Http);
    assert_eq!(SinkType::try_from("stdout")?, SinkType::Stdout);

    assert!(SinkType::from_str("invalid").is_err());
    assert!(SinkType::try_from("invalid").is_err());
    Ok(())
}

#[test]
fn test_sink_type_serde_serialization() -> Result<()> {
    assert_eq!(serde_json::to_string(&SinkType::Serial)?, "\"serial\"");
    assert_eq!(serde_json::to_string(&SinkType::Http)?, "\"http\"");
    assert_eq!(serde_json::to_string(&SinkType::Stdout)?, "\"stdout\"");
    Ok(())
}

#[test]
fn test_sink_type_serde_deserialization() -> Result<()> {
    assert_eq!(
        serde_json::from_str::<SinkType>("\"serial\"")?,
        SinkType::Serial
    );
    assert_eq!(
        serde_json::from_str::<SinkType>("\"http\"")?,
        SinkType::Http
    );
    assert_eq!(
        serde_json::from_str::<SinkType>("\"stdout\"")?,
        SinkType::Stdout
    );

    assert!(serde_json::from_str::<SinkType>("\"unknown\"").is_err());
    Ok(())
}
