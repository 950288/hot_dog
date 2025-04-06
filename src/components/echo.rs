use dioxus::prelude::*;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.scss");

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(format!("Hello, {:?}", data).to_string());
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

// Echo the user input on the server.
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<Vec<Vec<f32>>, ServerFnError> {
    use candle_core::{Device, Tensor};

    let device = Device::Cpu;

    let a = Tensor::randn(0f32, 1., (2, 3), &device)?;
    let b = Tensor::randn(0f32, 1., (3, 4), &device)?;

    let c: Vec<Vec<f32>> = a.matmul(&b)?.to_vec2::<f32>()?;
    Ok(c)
}
