use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Request, Response},
    http_component,
};

/// Send an HTTP request and return the response.
#[http_component]
async fn send_outbound(_req: Request) -> Result<impl IntoResponse> {
    let resp: Response = spin_sdk::http::send(Request::get(
        "https://api.github.com/repos/rajatjindal/spin-plugins/contents/manifests/kube/kube@0.2.0.json?ref=heads%2Fkube-v0.2.0",
    ))
    .await?;
    let resp = resp
        .into_builder()
        .header("spin-component", "rust-outbound-http")
        .build();
    println!("{resp:?}");
    Ok(resp)
}
