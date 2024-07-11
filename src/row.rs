use chrono::{DateTime, Duration, NaiveDate, Utc};
use serde::Deserialize;
use tiberius::{ColumnData, Row};

use crate::error::RowError;

#[allow(dead_code)]
pub trait RowExt {
    #[allow(deprecated)]
    fn from_row(row: Row) -> Result<Self, RowError>
    where
        Self: Sized + Deserialize<'static>,
    {
        let mut yml_str = "".to_string();
        let cols = row
            .columns()
            .iter()
            .map(|c| c.name().to_string())
            .collect::<Vec<_>>();
        for (col, val) in cols.iter().zip(row.into_iter()) {
            match val {
                ColumnData::I64(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                ColumnData::I32(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                ColumnData::I16(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                ColumnData::U8(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                ColumnData::F32(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                ColumnData::F64(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                ColumnData::Bit(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                ColumnData::String(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: {}\n", col, "")),
                },
                ColumnData::Numeric(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                ColumnData::DateTime(v) => match v {
                    Some(v) => {
                        let base_date = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
                        let days_duration = Duration::days(v.days().into());
                        let seconds = v.seconds_fragments() as f64 / 300.0; // Assuming 300 fragments per second
                        let seconds_duration = Duration::seconds(seconds.trunc() as i64);
                        let nano_seconds_duration =
                            Duration::nanoseconds(((seconds.fract()) * 1_000_000_000.0) as i64);

                        let naive_datetime = base_date.and_hms_opt(0, 0, 0).unwrap()
                            + days_duration
                            + seconds_duration
                            + nano_seconds_duration;

                        yml_str.push_str(
                            format!(
                                "{}: {}\n",
                                col,
                                DateTime::<Utc>::from_utc(naive_datetime, Utc)
                            )
                            .as_str(),
                        );
                    }
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                _ => {
                    return Err(RowError::DataTypeError(format!(
                        "Data type not supported: {:?}",
                        val
                    )));
                }
            }
        }
        let deserialized: Self = serde_yaml::from_str(Box::leak(Box::new(yml_str)))?;
        Ok(deserialized)
    }
}

// #[cfg(test)]
// mod tests {
//     use serde::Serialize;
//     use tiberius::Query;

//     use crate::state::AppState;

//     use super::*;
//     #[tokio::test]
//     async fn row_ext_should_work() {
//         #[derive(Debug, Serialize, Deserialize)]
//         struct SerialNumber {
//             #[serde(rename = "ID")]
//             id: i64,
//             #[serde(rename = "SerialType")]
//             serialtype: String,
//             #[serde(rename = "CurrentSerialNumber")]
//             serialnumber: String,
//         }
//         impl RowExt for SerialNumber {}
//         let mut state = AppState::new().await.unwrap();

//         let mut select = Query::new("select * from TTC..SerialNumbers where SerialType = @P1");
//         select.bind("ZTE");

//         let stream = select.query(&mut state.dbclient).await.unwrap();

//         let res = stream.into_first_result().await.unwrap();
//         let res = res
//             .into_iter()
//             .map(|r| SerialNumber::from_row(r).unwrap())
//             .collect::<Vec<_>>();
//         println!("{:?}", res);
//     }
// }
