use lolicon_api::Request;
use lolicon_api::R18;
use reqwest::blocking::get;
use serde_json::Value;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let req = Request::default()
        .r18(R18::Mixin);

    let url = String::from(req);

    let raw_result = get(url)?.text()?;

    let result: Value = serde_json::from_str(&raw_result)?;

    let original = result.pointer("/data/0/urls/original");

    if let Some(Value::String(ref image_url)) = original {
        let image_req = get(image_url)?;
        let file_name = image_req.url()
            .path_segments()
            .and_then(|path| path.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("temp.bin");
        let mut file = std::fs::File::create(file_name)?;
        file.write_all(image_req.bytes()?.as_ref())?;
    }

    Ok(())
}
