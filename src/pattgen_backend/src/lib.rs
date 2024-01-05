use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod, HttpHeader,
};
use base64::encode;

const API_KEY: &str = "";

#[ic_cdk::update]
async fn audio_start_gen(prompt: String) -> String {
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
async fn audio_check_gen(id: String) -> String {
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

#[ic_cdk::update]
async fn image_start_gen(prompt: String) -> String {
    // Constructing the JSON string manually to match the new payload structure
    let json_string = format!(
        r#"{{ 
            "version": "ac732df83cea7fff18b8472768c88ad041fa750ff7682a21affe81863cbe77e4",
            "input": {{
                "width": 512,
                "height": 512,
                "prompt": "{}",
                "scheduler": "K_EULER",
                "num_outputs": 1,
                "guidance_scale": 7.5,
                "num_inference_steps": 50
            }}
        }}"#,
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
async fn image_check_gen(get_url: String) -> String {
    let request_headers = vec![
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Token {}", API_KEY),
        },
        // Add other headers here as needed
    ];

    let request = CanisterHttpRequestArgument {
        url: get_url,
        method: HttpMethod::GET,
        body: None, // No body needed for GET request
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
async fn fetch_image_as_base64(get_url: String) -> String {
    let request_headers = vec![
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Token {}", API_KEY),
        },
        // Add other headers here as needed
    ];

    let request = CanisterHttpRequestArgument {
        url: get_url,
        method: HttpMethod::GET,
        body: None, // No body needed for GET request
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    match http_request(request).await {
        Ok((response,)) => {
            // Convert the response body (image data) to a Base64 string
            let base64_image = encode(response.body);
            return format!("data:image/png;base64,{}", base64_image);
        }
        Err((r, m)) => {
            let error_message = 
                format!("API_ERR. Failed to fetch image. RejectionCode: {r:?}, Error: {m}");
            return error_message;
        }
    }
}

#[ic_cdk::update]
async fn fetch_audio_raw(get_url: String) -> Vec<u8> {
    let request_headers = vec![
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Token {}", API_KEY),
        },
        // Add other headers here as needed
    ];

    let request = CanisterHttpRequestArgument {
        url: get_url,
        method: HttpMethod::GET,
        body: None, // No body needed for GET request
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    match http_request(request).await {
        Ok((response,)) => {
            response.body
        }
        Err((r, m)) => {
            // Log the error or handle it as needed
            ic_cdk::api::print(format!("API_ERR. Failed to fetch audio. RejectionCode: {r:?}, Error: {m}"));
            Vec::new() // Return an empty Vec<u8>
        }
    }
}
