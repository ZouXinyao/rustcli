use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut rdr = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = rdr.headers()?.clone();
    println!("{:?}", headers);
    for result in rdr.records() {
        let record = result?;
        // headers.iter(): headers的迭代器
        // record.iter(): record的迭代器
        // zip: 把两个迭代器合并成一个元组迭代器(headers, record)
        // collect: 把迭代器转换为一个Json对象 headers: record
        let json_val = headers.iter().zip(record.iter()).collect::<Value>();
        println!("{:?}", record);
        ret.push(json_val);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    std::fs::write(output, json)?;

    Ok(())
}
