// #[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
// #[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
// #[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::{glb_parser::parse_glb, mesh::Mesh};

// #[cfg(target_arch = "wasm32")]
pub async fn fetch_model_data(
    path: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture: &wgpu::BindGroupLayout,
    transform: &wgpu::BindGroupLayout,
) -> Result<Vec<Mesh>, String> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(path, &opts)
        .map_err(|_| "Failed to create request".to_string())?;
    let window = web_sys::window().ok_or_else(|| "Failed to get window".to_string())?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Failed to fetch".to_string())?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|_| "Failed to parse response".to_string())?;
    let buffer = JsFuture::from(
        resp.array_buffer()
            .map_err(|_| "Failed to convert response to buffer".to_string())?,
    )
    .await
    .map_err(|_| "Failed to convert buffer to future".to_string())?;
    let buffer = js_sys::Uint8Array::new(&buffer).to_vec();

    parse_glb(buffer, device, queue, texture, transform).map_err(|err| err.to_string())
}
