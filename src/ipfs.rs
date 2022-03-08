use std::process::Command;
use super::commands;
use reqwest;
use std::collections::HashMap;
use serde_json::{Value};
/*
async fn add_file() -> Result<String, Box<dyn std::error::Error>> {
    let client = IpfsClient::default();

    match client.add().await {
        Ok(res) => return Ok(res.hash()),
        Err(e) => return Err(e.into()),
    }
}
**/


pub fn start_daemon() {
    /*    Command::new("ipfs swarm peers")
        .arg("daemon")
        .output()
        .expect("failed to execute process");
    **/

    if true {
    //let Ok(_) = commands::run("ipfs daemon", "cmd") {
    commands::run("ipfs daemon");
    } else {
        println!("failed to start daemon assuming its already ruiing");
    }

    commands::run("ipfs swarm peers").unwrap();
    /*    Command::new("ipfs")
        .args(&args)
        .spawn()
        .expect("failed to execute process");
    **/
       // println!("{}", error);   
    //TODO: add a try catch and print failed to satrt ipfs make sure its installed
}
    
pub fn add_bootstrap_peers() {
    Command::new("ipfs")
        .args(&["swarm", "connect", "/ip4/149.56.89.144/tcp/4001/p2p/12D3KooWDiybBBYDvEEJQmNEp1yJeTgVr6mMgxqDrm9Gi8AKeNww"])
        .spawn()
        .expect("failed to execute process");
}

pub fn upload_file(file_path: &str) {

}

pub async fn get_pins() -> String {
    let client = reqwest::Client::new();
    let mut response = client.get("http://localhost:5001/api/v0/pin/ls").send().await.unwrap();
    let mut body = String::new();
    let body = response.text().await.unwrap();
    return body;
}

pub async fn get_data() -> String {
    //let result = reqwest::post("http://127.0.0.1:5001/api/v0/name/resolve?arg=k51qzi5uqu5dkrbpaj9wzcinngt6u0aijp8djmalb75eegp8ne2pmpppje55j6").await;
    let mut map = HashMap::new();
    map.insert("path", "".to_string());
    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:5001/api/v0/name/resolve?arg=k51qzi5uqu5dkrbpaj9wzcinngt6u0aijp8djmalb75eegp8ne2pmpppje55j6")
    .send().await;


    if let Ok(response) = res {
        if let Ok(text) = response.text().await {
            let result: serde_json::Result<Value> = serde_json::from_str(&text);
            if let Ok(j) = result {
                println!("{:?}", text);
                println!("received: {}", j["Path"]);
                map.insert("Path", j["Path"].to_string());
            } else {
                panic!("Could not convert text to json");
            }
        } else {
            panic!("Could not extract text from response");
        }
    } else {
        panic!("No response from server");
    }

    println!("{:?}", map["Path"]);

    let res = client.post("http://127.0.0.1:5001/api/v0/cid/base32?arg=".to_string()+&map["Path"][7..53]).send().await;

    let text = res.unwrap().text().await;
    let text = text.unwrap();
    let smt : serde_json::Result<Value>=  serde_json::from_str(&text);
    let smt = smt.unwrap();

    let encoded = smt["Formatted"].to_string();

    println!("{:?}", encoded);
    let res = client.post("http://127.0.0.1:5001/api/v0/get?arg=".to_string()+&encoded[1..60]).send().await;

    res.unwrap().text().await.unwrap()[131..].to_string()
 }
