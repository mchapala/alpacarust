use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;

pub(crate) struct HttpRequest {
    url: String,
    method: String,
}

#[derive(Debug, Clone)]
struct HttpResponse {
    status_code: u8,
    map: HashMap<String, String>,
}

impl HttpRequest {
    async fn get(&self) -> Result<HttpResponse, Box<dyn Error>> {
        let resp = reqwest::get(&self.url).await?;
        let status_code = resp.status();
        let json_map = resp.json::<HashMap<String, String>>().await?;
        debug!("{:#?}", json_map);

        Ok(HttpResponse {
            status_code: 200,
            map: json_map,
        })
    }
}

#[ignore]
#[test]
fn test_http_request() {
    let request = HttpRequest {
        url: "https://www.rust-lang.org".to_string(),
        method: "get".to_string(),
    };

    let _response = request.get();

    //println!("{:?}", _response);
}
