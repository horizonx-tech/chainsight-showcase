// Auto-generated code from manifest.
// You update the structure as needed.
// The existence of the SnapshotValue structure must be maintained.
use candid::{Decode, Encode};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
pub type SnapshotValue = Vec<SnapshotValue2>;

#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]
pub struct SnapshotValue2 {
    pub market_cap: u64,
    #[serde(deserialize_with = "de_f_or_u64")]
    pub circulating_supply: f64,
    #[serde(deserialize_with = "de_f_or_u64")]
    pub total_supply: f64,
}

fn de_f_or_u64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    let v: Value = Deserialize::deserialize(deserializer)?;
    match v {
        Value::Number(n) => match n.as_f64() {
            Some(f) => Ok(f),
            None => Ok(n.as_u64().unwrap() as f64),
        },
        _ => panic!("Unexpected value"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deserialize() {
        let out: SnapshotValue2 = serde_json::from_str(
            r#"{
            "market_cap": 1,
            "circulating_supply": 1,
            "total_supply": 1.0
        }"#,
        )
        .unwrap();
        assert_eq!(out.market_cap, 1);
        assert_eq!(out.circulating_supply, 1.0);
        assert_eq!(out.total_supply, 1.0);
    }
}
