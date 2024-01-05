use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};

#[ic_cdk::query]
async fn hello() -> String {
    return String::from("Hello World!");
}

#[ic_cdk::update]
async fn test() -> String {
    let request = CanisterHttpRequestArgument {
        url: String::from("https://httpbin.org/get"),
        method: HttpMethod::GET,
        body: None,               //optional for request
        max_response_bytes: None, //optional for request
        transform: None,          //optional for request
        headers: vec![],
    };

    match http_request(request).await {
        Ok((response,)) => {
            let body = String::from_utf8(response.body).unwrap();
            return body;
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
            return message;
        }
    }
}
