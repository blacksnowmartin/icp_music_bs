#![allow(clippy::collapsible_else_if)]

// #[macro_use]
// extern crate ic_cdk_macros;
// #[macro_use]
// extern crate serde;

use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod, HttpHeader,
};
use base64::encode;
use serde::Deserialize;

mod http;

const API_KEY: &str = "...";

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


/**
 * 
 * 
 * SMART contract
 * 
 * 
 **/ 
 use std::borrow::Cow;
 use std::cell::RefCell;
 use std::collections::{HashMap, HashSet};
 use std::convert::TryFrom;
 use std::iter::FromIterator;
 use std::mem;
 use std::num::TryFromIntError;
 use std::result::Result as StdResult;
 
 use candid::{CandidType, Encode, Principal};
 use ic_cdk::{
     api::{self, call},
     export::candid,
     storage,
 };
 use ic_certified_map::Hash;
 use include_base64::include_base64;
 
//  mod http;
 
 const MGMT: Principal = Principal::from_slice(&[]);
 
 thread_local! {
     static STATE: RefCell<State> = RefCell::default();
 }
 
 #[derive(CandidType, Deserialize)]
 struct StableState {
     state: State,
     hashes: Vec<(String, Hash)>,
 }
 
 #[ic_cdk::pre_upgrade]
 fn pre_upgrade() {
     let state = STATE.with(|state| mem::take(&mut *state.borrow_mut()));
     let hashes = http::HASHES.with(|hashes| mem::take(&mut *hashes.borrow_mut()));
     let hashes = hashes.iter().map(|(k, v)| (k.clone(), *v)).collect();
     let stable_state = StableState { state, hashes };
     storage::stable_save((stable_state,)).unwrap();
 }
 #[ic_cdk::post_upgrade]
 fn post_upgrade() {
     let (StableState { state, hashes },) = storage::stable_restore().unwrap();
     STATE.with(|state0| *state0.borrow_mut() = state);
     let hashes = hashes.into_iter().collect();
     http::HASHES.with(|hashes0| *hashes0.borrow_mut() = hashes);
 }
 
 #[ic_cdk::init]
 fn init() {
     STATE.with(|state| {
         let mut state = state.borrow_mut();
         state.custodians = HashSet::from_iter([api::caller()]);
         state.name = "AudioPrism".to_string();
         state.symbol = "AAPP".to_string();
         state.logo = None;
     });
 }
 
 #[derive(CandidType, Deserialize)]
 enum Error {
     Unauthorized,
     InvalidTokenId,
     ZeroAddress,
     Other,
 }
 
 impl From<TryFromIntError> for Error {
     fn from(_: TryFromIntError) -> Self {
         Self::InvalidTokenId
     }
 }
 
 type Result<T = u128, E = Error> = StdResult<T, E>;
 
 // --------------
 // base interface
 // --------------
 
 #[ic_cdk::query(name = "balanceOfDip721")]
 fn balance_of(user: Principal) -> u64 {
     STATE.with(|state| {
         state
             .borrow()
             .nfts
             .iter()
             .filter(|n| n.owner == user)
             .count() as u64
     })
 }
 
 #[ic_cdk::query(name = "ownerOfDip721")]
 fn owner_of(token_id: u64) -> Result<Principal> {
     STATE.with(|state| {
         let owner = state
             .borrow()
             .nfts
             .get(usize::try_from(token_id)?)
             .ok_or(Error::InvalidTokenId)?
             .owner;
         Ok(owner)
     })
 }
 
 #[ic_cdk::update(name = "transferFromDip721")]
 fn transfer_from(from: Principal, to: Principal, token_id: u64) -> Result {
     STATE.with(|state| {
         let mut state = state.borrow_mut();
         let state = &mut *state;
         let nft = state
             .nfts
             .get_mut(usize::try_from(token_id)?)
             .ok_or(Error::InvalidTokenId)?;
         let caller = api::caller();
         if nft.owner != caller
             && nft.approved != Some(caller)
             && !state
                 .operators
                 .get(&from)
                 .map(|s| s.contains(&caller))
                 .unwrap_or(false)
             && !state.custodians.contains(&caller)
         {
             Err(Error::Unauthorized)
         } else if nft.owner != from {
             Err(Error::Other)
         } else {
             nft.approved = None;
             nft.owner = to;
             Ok(state.next_txid())
         }
     })
 }
 
 #[ic_cdk::update(name = "safeTransferFromDip721")]
 fn safe_transfer_from(from: Principal, to: Principal, token_id: u64) -> Result {
     if to == MGMT {
         Err(Error::ZeroAddress)
     } else {
         transfer_from(from, to, token_id)
     }
 }
 
 #[ic_cdk::query(name = "supportedInterfacesDip721")]
 fn supported_interfaces() -> &'static [InterfaceId] {
     &[
         InterfaceId::TransferNotification,
         // InterfaceId::Approval, // Psychedelic/DIP721#5
         InterfaceId::Burn,
         InterfaceId::Mint,
     ]
 }
 
 #[derive(CandidType, Deserialize, Clone)]
 struct LogoResult {
     logo_type: Cow<'static, str>,
     data: Cow<'static, str>,
 }
 
 #[export_name = "canister_query logoDip721"]
 fn logo() /* -> &'static LogoResult */
 {
     ic_cdk::setup();
     STATE.with(|state| call::reply((state.borrow().logo.as_ref().unwrap_or(&DEFAULT_LOGO),)))
 }
 
 #[ic_cdk::query(name = "nameDip721")]
 fn name() -> String {
     STATE.with(|state| state.borrow().name.clone())
 }
 
 #[ic_cdk::query(name = "symbolDip721")]
 fn symbol() -> String {
     STATE.with(|state| state.borrow().symbol.clone())
 }
 
 const DEFAULT_LOGO: LogoResult = LogoResult {
     data: Cow::Borrowed("iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAApgAAAKYB3X3/OAAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAANCSURBVEiJtZZPbBtFFMZ/M7ubXdtdb1xSFyeilBapySVU8h8OoFaooFSqiihIVIpQBKci6KEg9Q6H9kovIHoCIVQJJCKE1ENFjnAgcaSGC6rEnxBwA04Tx43t2FnvDAfjkNibxgHxnWb2e/u992bee7tCa00YFsffekFY+nUzFtjW0LrvjRXrCDIAaPLlW0nHL0SsZtVoaF98mLrx3pdhOqLtYPHChahZcYYO7KvPFxvRl5XPp1sN3adWiD1ZAqD6XYK1b/dvE5IWryTt2udLFedwc1+9kLp+vbbpoDh+6TklxBeAi9TL0taeWpdmZzQDry0AcO+jQ12RyohqqoYoo8RDwJrU+qXkjWtfi8Xxt58BdQuwQs9qC/afLwCw8tnQbqYAPsgxE1S6F3EAIXux2oQFKm0ihMsOF71dHYx+f3NND68ghCu1YIoePPQN1pGRABkJ6Bus96CutRZMydTl+TvuiRW1m3n0eDl0vRPcEysqdXn+jsQPsrHMquGeXEaY4Yk4wxWcY5V/9scqOMOVUFthatyTy8QyqwZ+kDURKoMWxNKr2EeqVKcTNOajqKoBgOE28U4tdQl5p5bwCw7BWquaZSzAPlwjlithJtp3pTImSqQRrb2Z8PHGigD4RZuNX6JYj6wj7O4TFLbCO/Mn/m8R+h6rYSUb3ekokRY6f/YukArN979jcW+V/S8g0eT/N3VN3kTqWbQ428m9/8k0P/1aIhF36PccEl6EhOcAUCrXKZXXWS3XKd2vc/TRBG9O5ELC17MmWubD2nKhUKZa26Ba2+D3P+4/MNCFwg59oWVeYhkzgN/JDR8deKBoD7Y+ljEjGZ0sosXVTvbc6RHirr2reNy1OXd6pJsQ+gqjk8VWFYmHrwBzW/n+uMPFiRwHB2I7ih8ciHFxIkd/3Omk5tCDV1t+2nNu5sxxpDFNx+huNhVT3/zMDz8usXC3ddaHBj1GHj/As08fwTS7Kt1HBTmyN29vdwAw+/wbwLVOJ3uAD1wi/dUH7Qei66PfyuRj4Ik9is+hglfbkbfR3cnZm7chlUWLdwmprtCohX4HUtlOcQjLYCu+fzGJH2QRKvP3UNz8bWk1qMxjGTOMThZ3kvgLI5AzFfo379UAAAAASUVORK5CYII="),
     logo_type: Cow::Borrowed("image/png"),
 };
 
 #[ic_cdk::query(name = "totalSupplyDip721")]
 fn total_supply() -> u64 {
     STATE.with(|state| state.borrow().nfts.len() as u64)
 }
 
 #[export_name = "canister_query getMetadataDip721"]
 fn get_metadata(/* token_id: u64 */) /* -> Result<&'static MetadataDesc> */
 {
     ic_cdk::setup();
     let token_id = call::arg_data::<(u64,)>().0;
     let res: Result<()> = STATE.with(|state| {
         let state = state.borrow();
         let metadata = &state
             .nfts
             .get(usize::try_from(token_id)?)
             .ok_or(Error::InvalidTokenId)?
             .metadata;
         call::reply((Ok::<_, Error>(metadata),));
         Ok(())
     });
     if let Err(e) = res {
         call::reply((Err::<MetadataDesc, _>(e),));
     }
 }
 
 #[derive(CandidType)]
 struct ExtendedMetadataResult<'a> {
     metadata_desc: MetadataDescRef<'a>,
     token_id: u64,
 }
 
 #[export_name = "canister_update getMetadataForUserDip721"]
 fn get_metadata_for_user(/* user: Principal */) /* -> Vec<ExtendedMetadataResult> */
 {
     ic_cdk::setup();
     let user = call::arg_data::<(Principal,)>().0;
     STATE.with(|state| {
         let state = state.borrow();
         let metadata: Vec<_> = state
             .nfts
             .iter()
             .filter(|n| n.owner == user)
             .map(|n| ExtendedMetadataResult {
                 metadata_desc: &n.metadata,
                 token_id: n.id,
             })
             .collect();
         call::reply((metadata,));
     });
 }
 
 // ----------------------
 // notification interface
 // ----------------------
 
 #[ic_cdk::update(name = "transferFromNotifyDip721")]
 fn transfer_from_notify(from: Principal, to: Principal, token_id: u64, data: Vec<u8>) -> Result {
     let res = transfer_from(from, to, token_id)?;
     if let Ok(arg) = Encode!(&api::caller(), &from, &token_id, &data) {
         // Using call_raw ensures we don't need to await the future for the call to be executed.
         // Calling an arbitrary function like this means that a malicious recipient could call 
         // transferFromNotifyDip721 in their onDIP721Received function, resulting in an infinite loop.
         // This will trap eventually, but the transfer will have already been completed and the state-change persisted.
         // That means the original transfer must reply before that happens, or the caller will be
         // convinced that the transfer failed when it actually succeeded. So we don't await the call,
         // so that we'll reply immediately regardless of how long the notification call takes.
         let _ = api::call::call_raw(to, "onDIP721Received", &arg, 0);
     }
     Ok(res)
 }
 
 #[ic_cdk::update(name = "safeTransferFromNotifyDip721")]
 fn safe_transfer_from_notify(
     from: Principal,
     to: Principal,
     token_id: u64,
     data: Vec<u8>,
 ) -> Result {
     if to == MGMT {
         Err(Error::ZeroAddress)
     } else {
         transfer_from_notify(from, to, token_id, data)
     }
 }
 
 // ------------------
 // approval interface
 // ------------------
 
 #[ic_cdk::update(name = "approveDip721")]
 fn approve(user: Principal, token_id: u64) -> Result {
     STATE.with(|state| {
         let mut state = state.borrow_mut();
         let state = &mut *state;
         let caller = api::caller();
         let nft = state
             .nfts
             .get_mut(usize::try_from(token_id)?)
             .ok_or(Error::InvalidTokenId)?;
         if nft.owner != caller
             && nft.approved != Some(caller)
             && !state
                 .operators
                 .get(&user)
                 .map(|s| s.contains(&caller))
                 .unwrap_or(false)
             && !state.custodians.contains(&caller)
         {
             Err(Error::Unauthorized)
         } else {
             nft.approved = Some(user);
             Ok(state.next_txid())
         }
     })
 }
 
 #[ic_cdk::update(name = "setApprovalForAllDip721")]
 fn set_approval_for_all(operator: Principal, is_approved: bool) -> Result {
     STATE.with(|state| {
         let mut state = state.borrow_mut();
         let caller = api::caller();
         if operator != caller {
             let operators = state.operators.entry(caller).or_default();
             if operator == MGMT {
                 if !is_approved {
                     operators.clear();
                 } else {
                     // cannot enable everyone as an operator
                 }
             } else {
                 if is_approved {
                     operators.insert(operator);
                 } else {
                     operators.remove(&operator);
                 }
             }
         }
         Ok(state.next_txid())
     })
 }
 
 // #[query(name = "getApprovedDip721")] // Psychedelic/DIP721#5
 fn _get_approved(token_id: u64) -> Result<Principal> {
     STATE.with(|state| {
         let approved = state
             .borrow()
             .nfts
             .get(usize::try_from(token_id)?)
             .ok_or(Error::InvalidTokenId)?
             .approved
             .unwrap_or_else(api::caller);
         Ok(approved)
     })
 }
 
 #[ic_cdk::query(name = "isApprovedForAllDip721")]
 fn is_approved_for_all(operator: Principal) -> bool {
     STATE.with(|state| {
         state
             .borrow()
             .operators
             .get(&api::caller())
             .map(|s| s.contains(&operator))
             .unwrap_or(false)
     })
 }
 
 // --------------
 // mint interface
 // --------------
 
 #[ic_cdk::update(name = "mintDip721")]
 fn mint(
     to: Principal,
     metadata: MetadataDesc,
     blob_content: Vec<u8>,
 ) -> Result<MintResult, ConstrainedError> {
     let (txid, tkid) = STATE.with(|state| {
         let mut state = state.borrow_mut();
        //  if !state.custodians.contains(&api::caller()) {
        //      return Err(ConstrainedError::Unauthorized);
        //  }
         let new_id = state.nfts.len() as u64;
         let nft = Nft {
             owner: to,
             approved: None,
             id: new_id,
             metadata,
             content: blob_content,
         };
         state.nfts.push(nft);
         Ok((state.next_txid(), new_id))
     })?;
     http::add_hash(tkid);
     Ok(MintResult {
         id: txid,
         token_id: tkid,
     })
 }
 
 // --------------
 // burn interface
 // --------------
 
 #[ic_cdk::update(name = "burnDip721")]
 fn burn(token_id: u64) -> Result {
     STATE.with(|state| {
         let mut state = state.borrow_mut();
         let nft = state
             .nfts
             .get_mut(usize::try_from(token_id)?)
             .ok_or(Error::InvalidTokenId)?;
         if nft.owner != api::caller() {
             Err(Error::Unauthorized)
         } else {
             nft.owner = MGMT;
             Ok(state.next_txid())
         }
     })
 }
 
 #[derive(CandidType, Deserialize, Default)]
 struct State {
     nfts: Vec<Nft>,
     custodians: HashSet<Principal>,
     operators: HashMap<Principal, HashSet<Principal>>, // owner to operators
     logo: Option<LogoResult>,
     name: String,
     symbol: String,
     txid: u128,
 }
 
 #[derive(CandidType, Deserialize)]
 struct Nft {
     owner: Principal,
     approved: Option<Principal>,
     id: u64,
     metadata: MetadataDesc,
     content: Vec<u8>,
 }
 
 type MetadataDesc = Vec<MetadataPart>;
 type MetadataDescRef<'a> = &'a [MetadataPart];
 
 #[derive(CandidType, Deserialize)]
 struct MetadataPart {
     purpose: MetadataPurpose,
     key_val_data: HashMap<String, MetadataVal>,
     data: Vec<u8>,
 }
 
 #[derive(CandidType, Deserialize, PartialEq)]
 enum MetadataPurpose {
     Preview,
     Rendered,
 }
 
 #[derive(CandidType, Deserialize)]
 struct MintResult {
     token_id: u64,
     id: u128,
 }
 
 #[allow(clippy::enum_variant_names)]
 #[derive(CandidType, Deserialize)]
 enum MetadataVal {
     TextContent(String),
     BlobContent(Vec<u8>),
     NatContent(u128),
     Nat8Content(u8),
     Nat16Content(u16),
     Nat32Content(u32),
     Nat64Content(u64),
 }
 
 impl State {
     fn next_txid(&mut self) -> u128 {
         let txid = self.txid;
         self.txid += 1;
         txid
     }
 }
 
 #[derive(CandidType, Deserialize)]
 enum InterfaceId {
     Approval,
     TransactionHistory,
     Mint,
     Burn,
     TransferNotification,
 }
 
 #[derive(CandidType, Deserialize)]
 enum ConstrainedError {
     Unauthorized,
 }
 
 #[ic_cdk::update]
 fn set_name(name: String) -> Result<()> {
     STATE.with(|state| {
         let mut state = state.borrow_mut();
         if state.custodians.contains(&api::caller()) {
             state.name = name;
             Ok(())
         } else {
             Err(Error::Unauthorized)
         }
     })
 }
 
 #[ic_cdk::update]
 fn set_symbol(sym: String) -> Result<()> {
     STATE.with(|state| {
         let mut state = state.borrow_mut();
         if state.custodians.contains(&api::caller()) {
             state.symbol = sym;
             Ok(())
         } else {
             Err(Error::Unauthorized)
         }
     })
 }
 
 #[ic_cdk::update]
 fn set_logo(logo: Option<LogoResult>) -> Result<()> {
     STATE.with(|state| {
         let mut state = state.borrow_mut();
         if state.custodians.contains(&api::caller()) {
             state.logo = logo;
             Ok(())
         } else {
             Err(Error::Unauthorized)
         }
     })
 }
 
 #[ic_cdk::update]
 fn set_custodian(user: Principal, custodian: bool) -> Result<()> {
     STATE.with(|state| {
         let mut state = state.borrow_mut();
         if state.custodians.contains(&api::caller()) {
             if custodian {
                 state.custodians.insert(user);
             } else {
                 state.custodians.remove(&user);
             }
             Ok(())
         } else {
             Err(Error::Unauthorized)
         }
     })
 }
 
 #[ic_cdk::query]
 fn is_custodian(principal: Principal) -> bool {
     STATE.with(|state| state.borrow().custodians.contains(&principal))
 }
 