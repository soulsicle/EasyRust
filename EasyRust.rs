use reqwest;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};

pub async fn webget(variable: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(variable).await?.text().await?;
    Ok(body)
}

pub fn write(variable1: &str, variable2: &str) -> io::Result<()> {
    let mut file = File::create(variable1)?;
    file.write_all(variable2.as_bytes())?;
    Ok(())
}

pub fn read(variable: &str) -> io::Result<String> {
    let mut file = File::open(variable)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub async fn webpost(variable: &str, variable2: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(variable).body(variable2).send().await?;
    let res0 = res as String;
    Ok(res0)
}

pub async fn delete(file: String) -> std::io::Result<()> {
    fs::remove_file(file)?;
    ok(())
}

pub fn breakonint(variable: i128) {
    let mut num = 0;
    loop {
        num = num+1;
        if num == variable {
            break
        }
    }
}