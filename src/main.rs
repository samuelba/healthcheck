use std::env;
use std::process::exit;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => "9000".to_string(),
    };
    let path = match env::var("API_PATH") {
        Ok(val) => val,
        Err(_) => "".to_string(),
    };

    let url = format!("http://localhost:{}/{}", port, path);
    let client = reqwest::Client::new();
    let res = client.get(url).send().await;
    match res {
        Ok(res) => {
            if res.status() < reqwest::StatusCode::OK
                || res.status() >= reqwest::StatusCode::MULTIPLE_CHOICES
            {
                exit(1)
            }
            exit(0)
        }
        Err(_) => exit(1),
    }
}
