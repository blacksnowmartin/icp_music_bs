use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod, HttpHeader,
};

const API_KEY: &str = "...";

// #[ic_cdk::update]
// async fn start_gen(prompt: String) -> String {
//     let request = CanisterHttpRequestArgument {
//         url: String::from("https://httpbin.org/get"),
//         method: HttpMethod::GET,
//         body: None,               //optional for request
//         max_response_bytes: None, //optional for request
//         transform: None,          //optional for request
//         headers: vec![],
//     };

//     match http_request(request).await {
//         Ok((response,)) => {
//             let body = String::from_utf8(response.body).unwrap();
//             return body;
//         }
//         Err((r, m)) => {
//             let message =
//                 format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
//             return message;
//         }
//     }
// }

#[ic_cdk::update]
async fn start_gen(prompt: String) -> String {
    // Constructing JSON string manually
    let json_string = format!(
        "{{ \"version\": \"b05b1dff1d8c6dc63d14b0cdb42135378dcb87f6373b0d3d341ede46e59e2b38\", \"input\": {{ \"model_version\": \"stereo-melody-large\", \"prompt\": \"{}\" }} }}",
        prompt
    );
    let request_body: Option<Vec<u8>> = Some(json_string.into_bytes());

    let request_headers = vec![
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Token {}", API_KEY),
        },
        // Add other headers here as needed
    ];

    let request = CanisterHttpRequestArgument {
        url: String::from("https://api.replicate.com/v1/predictions"),
        method: HttpMethod::POST,
        body: request_body,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    match http_request(request).await {
        Ok((response,)) => {
            let body = String::from_utf8(response.body).unwrap();
            return body;
        }
        Err((r, m)) => {
            let message = 
                format!("API_ERR. The http_request resulted in an error. RejectionCode: {r:?}, Error: {m}");
            return message;
        }
    }
}



#[ic_cdk::update]
async fn check_gen(id: String) -> String {
    let url = format!("https://api.replicate.com/v1/predictions/{}", id);

    let request_headers = vec![
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Token {}", API_KEY),
        },
        // Add other headers here as needed
    ];

    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None, // No body is needed for a GET request
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    match http_request(request).await {
        Ok((response,)) => {
            let body = String::from_utf8(response.body).unwrap();
            return body;
        }
        Err((r, m)) => {
            let message = format!("API_ERR. The http_request resulted in an error. RejectionCode: {r:?}, Error: {m}");
            return message;
        }
    }
}