use std::{
    collections::HashMap,
    io::Read,
    sync::{LazyLock, Mutex},
};
use webview2_com::Microsoft::Web::WebView2::Win32::*;
use windows::{
    Win32::{
        Foundation::HGLOBAL,
        System::Com::{IStream, STREAM_SEEK_SET, StructuredStorage::CreateStreamOnHGlobal},
    },
    core::*,
};

use crate::{constants, utils};

static FETCHED_SHEETS: LazyLock<Mutex<HashMap<String, Vec<u8>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

const OVERRIDE_MARKER: &str = "\n/* protox client override */\n";

fn path_matches(path: &str) -> bool {
    let file = path.rsplit('/').next().unwrap_or("");
    file.starts_with("style-") && file.ends_with(".css")
}

pub fn load(webview: &ICoreWebView2) {
    let patterns = [
        format!("*://{}/*style-*.css*", constants::TARGET_HOST),
        format!("*://*.{}/style-*.css*", constants::TARGET_HOST),
    ];
    for pattern in patterns {
        unsafe {
            let _ = webview.AddWebResourceRequestedFilter(PCWSTR(utils::create_utf_string(pattern).as_ptr()), COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL);
        }
    }
}

pub fn create_stream(bytes: &[u8]) -> Result<IStream> {
    unsafe {
        let stream = CreateStreamOnHGlobal(HGLOBAL::default(), true).unwrap();
        stream.Write(bytes.as_ptr() as *const _, bytes.len() as u32, None).unwrap();
        stream.Seek(0, STREAM_SEEK_SET, None).ok();
        Ok(stream)
    }
}

fn fetch_original(url: &str) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    let response = ureq::get(url).call().ok()?;
    response.into_body().as_reader().read_to_end(&mut buf).ok()?;
    (!buf.is_empty()).then_some(buf)
}

pub fn intercept(uri: &str) -> Option<Vec<u8>> {
    if !uri.contains(constants::TARGET_HOST) {
        return None;
    }
    let path = uri
        .split(&format!("{}/", constants::TARGET_HOST))
        .nth(1)?
        .split('?')
        .next()?
        .to_string();
    if !path_matches(&path) {
        return None;
    }

    let original = {
        let mut cache = FETCHED_SHEETS.lock().unwrap();
        if let Some(cached) = cache.get(&path) {
            return Some(append_override(cached));
        }
        let url = format!("https://{}/{}", constants::TARGET_HOST, path);
        let body = fetch_original(&url)?;
        cache.insert(path, body.clone());
        body
    };

    Some(append_override(&original))
}

fn append_override(original: &[u8]) -> Vec<u8> {
    let mut combined = original.to_vec();
    if let Ok(user_css) = std::fs::read(utils::settings_dir().join("css").join("style.css")) {
        combined.extend_from_slice(OVERRIDE_MARKER.as_bytes());
        combined.extend_from_slice(&user_css);
    }
    combined
}
