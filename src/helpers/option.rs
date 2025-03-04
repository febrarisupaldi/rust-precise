pub mod option_ts_seconds {
    use chrono::{NaiveDateTime, Duration};
    use serde::{self, Serializer, Deserializer};
    use serde::Deserialize;

    pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => serializer.serialize_some(&dt.and_utc().timestamp()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp: Option<i64> = Option::deserialize(deserializer)?;
        Ok(timestamp.map(|ts| NaiveDateTime::UNIX_EPOCH + Duration::seconds(ts)))
    }
}