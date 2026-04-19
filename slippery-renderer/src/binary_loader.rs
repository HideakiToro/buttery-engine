#[cfg(not(target_arch = "wasm32"))]
use std::{fs::File, io::Read};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit, RequestMode, Response};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("Failed to load data")]
    FailedToLoad,
}

pub async fn load_binary(path: &str) -> anyhow::Result<Vec<u8>> {
    #[cfg(target_arch = "wasm32")]
    {
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);

        let request =
            Request::new_with_str_and_init(path, &opts).map_err(|_| LoadError::FailedToLoad)?;
        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|_| LoadError::FailedToLoad)?;

        let resp: Response = resp_value.dyn_into().map_err(|_| LoadError::FailedToLoad)?;
        let buffer = JsFuture::from(resp.array_buffer().map_err(|_| LoadError::FailedToLoad)?)
            .await
            .map_err(|_| LoadError::FailedToLoad)?;
        let u8_array = js_sys::Uint8Array::new(&buffer);

        Ok(u8_array.to_vec())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = path.replace("./", "./src/");
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer.to_vec())
    }
}
