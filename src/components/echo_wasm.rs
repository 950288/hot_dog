use dioxus::prelude::*;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.scss");

#[component]
pub fn EchoWasm() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }
        div {
            id: "echo",
            h4 { "wasmFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

async fn echo_server(input: String) -> Result<String, ServerFnError> {
    // use candle_core::{Device, Tensor};

    // let device = Device::Cpu;

    // let a = Tensor::randn(0f32, 1., (2, 3), &device)?;
    // let b = Tensor::randn(0f32, 1., (3, 4), &device)?;

    // let c = a.matmul(&b)?.to_vec2::<f32>()?;
    let c = "fghjk";
    Ok(format!("Hello, {:?}", c).to_string())
}
