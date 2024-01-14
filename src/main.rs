use reqwest::blocking::{Client, ClientBuilder};

fn main() {
    let client = Client::new();
    let res = client.get("https://leahjia.com").send();
    if res.is_ok() {
        println!("Body: {:#?}", res.ok().unwrap());
    } else if res.is_err() {
        println!("Failed: {:#?}", res.unwrap_err());
    }

    let post_res = client.post("URL").body("{\"first_name\": \"firsty\",\"last_name\": \"lasty\"}").send();
    println!("Web post result: {:#?}", post_res.ok().unwrap());
}
