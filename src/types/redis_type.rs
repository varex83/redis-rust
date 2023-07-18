use crate::types::{ArrayType, EventChannelType, IntegerType, StringType};
use serde::{Deserialize, Serialize};

/// RedisType - is implementation of basic Redis' types
/// Derives `Serialize`/`Deserialize`
///
/// ## Variants
///
/// - `String(StringType)` -- Basic string
/// - `Integer(IntegerType)` -- Basic integer type
/// - `Array(ArrayType)` -- Array implementation
/// - `EventChannel(EventChannelType)` -- Channel type
/// - `Null` - Just nothing :D
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RedisType {
    String(StringType),
    Integer(IntegerType),
    Array(ArrayType),
    EventChannel(EventChannelType),
    Null,
    Ok,
}

impl RedisType {
    pub fn parse(str: String) -> Self {
        if str.starts_with('+') {
            RedisType::String(str[1..].to_string())
        } else if str.starts_with(':') {
            RedisType::Integer(str[1..].parse::<IntegerType>().unwrap())
        } else if str.starts_with('*') {
            let mut arr = ArrayType::new();
            let mut lines = str.lines();
            lines.next();
            for line in lines {
                arr.push(RedisType::parse(line.to_string()));
            }
            RedisType::Array(arr)
        } else if str.starts_with("$-1") {
            RedisType::Null
        } else if str.starts_with("+OK") {
            RedisType::Ok
        } else {
            RedisType::Null
        }
    }

    pub fn as_string_formatted(&self) -> String {
        match self {
            RedisType::String(str) => {
                let mut res = String::from("+");
                res.push_str(str);
                res.push_str("\r\n");
                res
            }
            RedisType::Integer(int) => {
                let mut res = String::from(":");
                res.push_str(&int.to_string());
                res.push_str("\r\n");
                res
            }
            RedisType::Array(arr) => {
                let mut res = String::from("*");
                res.push_str(&arr.len().to_string());
                res.push_str("\r\n");
                for item in arr {
                    res.push_str(&item.as_string_formatted().to_string());
                    res.push_str("\r\n");
                }
                res
            }
            RedisType::EventChannel(_ch) => "+OK\r\n".to_string(),
            RedisType::Null => "$-1\r\n".to_string(),
            RedisType::Ok => "+OK\r\n".to_string(),
        }
    }
}
