use rust_code_obfuscator::{obfuscate_flow, obfuscate_string};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, WebGlRenderingContext};

/// Fingerprint data structure containing all collected browser information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fingerprint {
    // Basic browser info
    pub user_agent: String,
    pub language: String,
    pub languages: Vec<String>,
    pub platform: String,
    pub cookie_enabled: bool,
    pub do_not_track: Option<String>,

    // Hardware info
    pub hardware_concurrency: Option<u32>,
    pub device_memory: Option<f64>,
    pub max_touch_points: u32,

    // Screen info
    pub screen_width: i32,
    pub screen_height: i32,
    pub screen_color_depth: i32,
    pub screen_pixel_depth: i32,
    pub screen_avail_width: i32,
    pub screen_avail_height: i32,
    pub device_pixel_ratio: f64,

    // Timezone info
    pub timezone: String,
    pub timezone_offset: i32,

    // Storage support
    pub local_storage: bool,
    pub session_storage: bool,
    pub indexed_db: bool,

    // Canvas fingerprint
    pub canvas_fingerprint: String,

    // WebGL info
    pub webgl_vendor: String,
    pub webgl_renderer: String,
    pub webgl_version: String,
    pub webgl_shading_language_version: String,
    pub webgl_extensions: Vec<String>,

    // Audio fingerprint
    pub audio_fingerprint: String,

    // Plugins & MIME types
    pub plugins: Vec<String>,
    pub mime_types: Vec<String>,

    // Connection info
    pub online: bool,

    // Final hash
    pub fingerprint_hash: String,
}

#[wasm_bindgen]
pub struct BrowserFingerprinter {
    fingerprint: Option<Fingerprint>,
}

#[wasm_bindgen]
impl BrowserFingerprinter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        BrowserFingerprinter { fingerprint: None }
    }

    /// Collect all fingerprint data
    #[wasm_bindgen]
    pub fn collect(&mut self) -> Result<String, JsValue> {
        obfuscate_flow!();
        let window = window().ok_or("No window object")?;
        obfuscate_flow!();
        let navigator = window.navigator();
        let screen = window.screen().map_err(|_| "No screen object")?;
        let document = window.document().ok_or("No document object")?;

        // Basic browser info
        let user_agent = navigator.user_agent().unwrap_or_default();
        let language = navigator.language().unwrap_or_default();
        let languages = get_languages(&navigator);
        let platform = navigator.platform().unwrap_or_default();
        let cookie_enabled = get_cookie_enabled(&navigator);
        let do_not_track = get_do_not_track(&navigator);

        // Hardware info
        let hardware_concurrency = Some(navigator.hardware_concurrency() as u32);
        let device_memory = get_device_memory();
        let max_touch_points = navigator.max_touch_points() as u32;

        // Screen info
        let screen_width = screen.width().unwrap_or(0);
        let screen_height = screen.height().unwrap_or(0);
        let screen_color_depth = screen.color_depth().unwrap_or(0);
        let screen_pixel_depth = screen.pixel_depth().unwrap_or(0);
        let screen_avail_width = screen.avail_width().unwrap_or(0);
        let screen_avail_height = screen.avail_height().unwrap_or(0);
        let device_pixel_ratio = window.device_pixel_ratio();

        // Timezone info
        let timezone = get_timezone();
        let timezone_offset = js_sys::Date::new_0().get_timezone_offset() as i32;

        // Storage support
        let local_storage = window.local_storage().ok().flatten().is_some();
        let session_storage = window.session_storage().ok().flatten().is_some();
        let indexed_db = check_indexed_db();

        // Canvas fingerprint
        obfuscate_flow!();
        let canvas_fingerprint = generate_canvas_fingerprint(&document)?;

        // WebGL info
        obfuscate_flow!();
        let (
            webgl_vendor,
            webgl_renderer,
            webgl_version,
            webgl_shading_language_version,
            webgl_extensions,
        ) = get_webgl_info(&document)?;

        // Audio fingerprint
        obfuscate_flow!();
        let audio_fingerprint = "audio-context-available".to_string();

        // Plugins & MIME types
        obfuscate_flow!();
        let plugins = get_plugins(&navigator);
        let mime_types = get_mime_types(&navigator);

        // Connection info
        let online = navigator.on_line();

        // Create fingerprint struct
        let mut fingerprint = Fingerprint {
            user_agent,
            language,
            languages,
            platform,
            cookie_enabled,
            do_not_track,
            hardware_concurrency,
            device_memory,
            max_touch_points,
            screen_width,
            screen_height,
            screen_color_depth,
            screen_pixel_depth,
            screen_avail_width,
            screen_avail_height,
            device_pixel_ratio,
            timezone,
            timezone_offset,
            local_storage,
            session_storage,
            indexed_db,
            canvas_fingerprint,
            webgl_vendor,
            webgl_renderer,
            webgl_version,
            webgl_shading_language_version,
            webgl_extensions,
            audio_fingerprint,
            plugins,
            mime_types,
            online,
            fingerprint_hash: String::new(),
        };

        // Generate final hash
        fingerprint.fingerprint_hash = generate_hash(&fingerprint);

        let json = serde_json::to_string_pretty(&fingerprint)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.fingerprint = Some(fingerprint);

        Ok(json)
    }

    /// Get just the fingerprint hash
    #[wasm_bindgen]
    pub fn get_hash(&self) -> Option<String> {
        self.fingerprint.as_ref().map(|f| f.fingerprint_hash.clone())
    }
}

fn get_languages(navigator: &web_sys::Navigator) -> Vec<String> {
    let arr = navigator.languages();
    let mut result = Vec::new();
    for i in 0..arr.length() {
        if let Some(lang) = arr.get(i).as_string() {
            result.push(lang);
        }
    }
    if result.is_empty() {
        if let Some(lang) = navigator.language() {
            result.push(lang);
        }
    }
    result
}

fn get_cookie_enabled(navigator: &web_sys::Navigator) -> bool {
    obfuscate_flow!();
    js_sys::Reflect::get(navigator, &JsValue::from_str(&obfuscate_string!("cookieEnabled")))
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

fn get_do_not_track(navigator: &web_sys::Navigator) -> Option<String> {
    obfuscate_flow!();
    let dnt = js_sys::Reflect::get(navigator, &JsValue::from_str(&obfuscate_string!("doNotTrack"))).ok()?;
    dnt.as_string()
}

fn get_device_memory() -> Option<f64> {
    obfuscate_flow!();
    let window = window()?;
    let navigator = window.navigator();
    let memory = js_sys::Reflect::get(&navigator, &JsValue::from_str(&obfuscate_string!("deviceMemory"))).ok()?;
    memory.as_f64()
}

fn get_timezone() -> String {
    // Try to get timezone name via Intl.DateTimeFormat
    if let Ok(intl) = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str(&obfuscate_string!("Intl"))) {
        if let Ok(dtf) = js_sys::Reflect::get(&intl, &JsValue::from_str(&obfuscate_string!("DateTimeFormat"))) {
            if let Ok(dtf_func) = dtf.dyn_into::<js_sys::Function>() {
                if let Ok(formatter) = dtf_func.call0(&JsValue::NULL) {
                    if let Ok(resolved) =
                        js_sys::Reflect::get(&formatter, &JsValue::from_str(&obfuscate_string!("resolvedOptions")))
                    {
                        if let Ok(resolved_func) = resolved.dyn_into::<js_sys::Function>() {
                            if let Ok(options) = resolved_func.call0(&formatter) {
                                if let Ok(tz) =
                                    js_sys::Reflect::get(&options, &JsValue::from_str(&obfuscate_string!("timeZone")))
                                {
                                    if let Some(tz_str) = tz.as_string() {
                                        return tz_str;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    "Unknown".to_string()
}

fn check_indexed_db() -> bool {
    if let Some(window) = window() {
        js_sys::Reflect::get(&window, &JsValue::from_str(&obfuscate_string!("indexedDB")))
            .map(|v| !v.is_undefined() && !v.is_null())
            .unwrap_or(false)
    } else {
        false
    }
}

fn generate_canvas_fingerprint(document: &web_sys::Document) -> Result<String, JsValue> {
    obfuscate_flow!();
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    canvas.set_width(280);
    canvas.set_height(60);

    obfuscate_flow!();
    let context = canvas
        .get_context("2d")?
        .ok_or("Failed to get 2d context")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Draw various shapes and text to create a unique fingerprint
    context.set_fill_style_str("#f60");
    context.fill_rect(0.0, 0.0, 62.0, 20.0);

    context.set_fill_style_str("#069");
    context.set_font(&obfuscate_string!("15px 'Arial'"));
    context.fill_text(&obfuscate_string!("Browser Fingerprint!"), 2.0, 15.0)?;

    context.set_fill_style_str("rgba(102, 204, 0, 0.7)");
    context.set_font(&obfuscate_string!("18px 'Times New Roman'"));
    context.fill_text(&obfuscate_string!("Canvas 2D Test"), 4.0, 45.0)?;

    // Draw some geometric shapes
    context.begin_path();
    context.arc(150.0, 30.0, 20.0, 0.0, std::f64::consts::PI * 2.0)?;
    context.set_fill_style_str("#ff0066");
    context.fill();

    context.begin_path();
    context.move_to(200.0, 10.0);
    context.line_to(220.0, 50.0);
    context.line_to(180.0, 50.0);
    context.close_path();
    context.set_fill_style_str("#3399ff");
    context.fill();

    // Get canvas data and hash it
    let data_url = canvas.to_data_url()?;
    let mut hasher = Sha256::new();
    hasher.update(data_url.as_bytes());
    let result = hasher.finalize();

    Ok(hex::encode(result))
}

fn get_webgl_info(
    document: &web_sys::Document,
) -> Result<(String, String, String, String, Vec<String>), JsValue> {
    obfuscate_flow!();
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    obfuscate_flow!();
    // Try WebGL1 first (easier to work with), then WebGL2
    let gl_result = canvas
        .get_context("webgl")
        .ok()
        .flatten()
        .or_else(|| canvas.get_context("experimental-webgl").ok().flatten());

    if let Some(gl_value) = gl_result {
        // Try to cast to WebGL1 context
        if let Ok(gl) = gl_value.dyn_into::<WebGlRenderingContext>() {
            return get_webgl1_info(gl);
        }
    }

    // Try WebGL2 as fallback using reflection-based approach
    if let Ok(Some(gl2_value)) = canvas.get_context("webgl2") {
        return get_webgl_info_via_reflection(gl2_value.into());
    }

    Ok((
        "Not available".to_string(),
        "Not available".to_string(),
        "Not available".to_string(),
        "Not available".to_string(),
        vec![],
    ))
}

fn get_webgl1_info(
    gl: WebGlRenderingContext,
) -> Result<(String, String, String, String, Vec<String>), JsValue> {
    // Try to get the debug info extension
    let debug_info = gl.get_extension("WEBGL_debug_renderer_info").ok().flatten();

    let (vendor, renderer) = if let Some(debug_ext) = debug_info {
        let unmasked_vendor_enum =
            js_sys::Reflect::get(&debug_ext, &JsValue::from_str("UNMASKED_VENDOR_WEBGL"))
                .ok()
                .and_then(|v| v.as_f64())
                .map(|v| v as u32);

        let unmasked_renderer_enum =
            js_sys::Reflect::get(&debug_ext, &JsValue::from_str("UNMASKED_RENDERER_WEBGL"))
                .ok()
                .and_then(|v| v.as_f64())
                .map(|v| v as u32);

        let vendor = unmasked_vendor_enum
            .and_then(|e| gl.get_parameter(e).ok())
            .and_then(|v| v.as_string())
            .unwrap_or_else(|| {
                gl.get_parameter(WebGlRenderingContext::VENDOR)
                    .ok()
                    .and_then(|v| v.as_string())
                    .unwrap_or_default()
            });

        let renderer = unmasked_renderer_enum
            .and_then(|e| gl.get_parameter(e).ok())
            .and_then(|v| v.as_string())
            .unwrap_or_else(|| {
                gl.get_parameter(WebGlRenderingContext::RENDERER)
                    .ok()
                    .and_then(|v| v.as_string())
                    .unwrap_or_default()
            });

        (vendor, renderer)
    } else {
        let vendor = gl
            .get_parameter(WebGlRenderingContext::VENDOR)
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
        let renderer = gl
            .get_parameter(WebGlRenderingContext::RENDERER)
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
        (vendor, renderer)
    };

    let version = gl
        .get_parameter(WebGlRenderingContext::VERSION)
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_default();

    let shading_version = gl
        .get_parameter(WebGlRenderingContext::SHADING_LANGUAGE_VERSION)
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_default();

    let extensions: Vec<String> = gl
        .get_supported_extensions()
        .map(|exts| {
            (0..exts.length())
                .filter_map(|i| exts.get(i).as_string())
                .collect()
        })
        .unwrap_or_default();

    Ok((vendor, renderer, version, shading_version, extensions))
}

fn get_webgl_info_via_reflection(
    gl: JsValue,
) -> Result<(String, String, String, String, Vec<String>), JsValue> {
    obfuscate_flow!();
    // WebGL constants
    const VENDOR: u32 = 0x1F00;
    const RENDERER: u32 = 0x1F01;
    const VERSION: u32 = 0x1F02;
    const SHADING_LANGUAGE_VERSION: u32 = 0x8B8C;

    obfuscate_flow!();
    // Get the getParameter function
    let get_parameter = js_sys::Reflect::get(&gl, &JsValue::from_str(&obfuscate_string!("getParameter")))?;
    let get_parameter_fn: js_sys::Function = get_parameter.dyn_into()?;

    // Try to get debug extension for unmasked vendor/renderer
    let get_extension = js_sys::Reflect::get(&gl, &JsValue::from_str(&obfuscate_string!("getExtension")))?;
    let get_extension_fn: js_sys::Function = get_extension.dyn_into()?;
    let debug_ext = get_extension_fn.call1(&gl, &JsValue::from_str(&obfuscate_string!("WEBGL_debug_renderer_info")))?;

    let (vendor, renderer) = if !debug_ext.is_null() && !debug_ext.is_undefined() {
        // UNMASKED_VENDOR_WEBGL = 0x9245 (37445), UNMASKED_RENDERER_WEBGL = 0x9246 (37446)
        let unmasked_vendor = get_parameter_fn
            .call1(&gl, &JsValue::from_f64(37445.0))?
            .as_string()
            .unwrap_or_default();
        let unmasked_renderer = get_parameter_fn
            .call1(&gl, &JsValue::from_f64(37446.0))?
            .as_string()
            .unwrap_or_default();
        (unmasked_vendor, unmasked_renderer)
    } else {
        let vendor = get_parameter_fn
            .call1(&gl, &JsValue::from_f64(VENDOR as f64))?
            .as_string()
            .unwrap_or_default();
        let renderer = get_parameter_fn
            .call1(&gl, &JsValue::from_f64(RENDERER as f64))?
            .as_string()
            .unwrap_or_default();
        (vendor, renderer)
    };

    let version = get_parameter_fn
        .call1(&gl, &JsValue::from_f64(VERSION as f64))?
        .as_string()
        .unwrap_or_default();

    let shading_version = get_parameter_fn
        .call1(&gl, &JsValue::from_f64(SHADING_LANGUAGE_VERSION as f64))?
        .as_string()
        .unwrap_or_default();

    // Get supported extensions
    let get_supported_extensions =
        js_sys::Reflect::get(&gl, &JsValue::from_str(&obfuscate_string!("getSupportedExtensions")))?;
    let get_supported_extensions_fn: js_sys::Function = get_supported_extensions.dyn_into()?;
    let extensions_array = get_supported_extensions_fn.call0(&gl)?;

    let extensions: Vec<String> = if let Ok(arr) = extensions_array.dyn_into::<js_sys::Array>() {
        (0..arr.length())
            .filter_map(|i| arr.get(i).as_string())
            .collect()
    } else {
        vec![]
    };

    Ok((vendor, renderer, version, shading_version, extensions))
}

fn get_plugins(navigator: &web_sys::Navigator) -> Vec<String> {
    let mut plugins = Vec::new();
    if let Ok(plugin_array) = navigator.plugins() {
        let length = plugin_array.length();
        for i in 0..length {
            // Use item() method instead of get()
            if let Some(plugin) = plugin_array.item(i) {
                let name = plugin.name();
                let desc = plugin.description();
                plugins.push(format!("{} ({})", name, desc));
            }
        }
    }
    plugins
}

fn get_mime_types(navigator: &web_sys::Navigator) -> Vec<String> {
    let mut mime_types = Vec::new();
    if let Ok(mime_array) = navigator.mime_types() {
        let length = mime_array.length();
        for i in 0..length {
            // Use item() method instead of get()
            if let Some(mime) = mime_array.item(i) {
                mime_types.push(mime.type_());
            }
        }
    }
    mime_types
}

fn generate_hash(fingerprint: &Fingerprint) -> String {
    let mut hasher = Sha256::new();

    // Hash all the fingerprint components
    hasher.update(&fingerprint.user_agent);
    hasher.update(&fingerprint.language);
    hasher.update(&fingerprint.platform);
    hasher.update(fingerprint.hardware_concurrency.unwrap_or(0).to_string());
    hasher.update(fingerprint.screen_width.to_string());
    hasher.update(fingerprint.screen_height.to_string());
    hasher.update(fingerprint.screen_color_depth.to_string());
    hasher.update(fingerprint.device_pixel_ratio.to_string());
    hasher.update(&fingerprint.timezone);
    hasher.update(fingerprint.timezone_offset.to_string());
    hasher.update(&fingerprint.canvas_fingerprint);
    hasher.update(&fingerprint.webgl_vendor);
    hasher.update(&fingerprint.webgl_renderer);

    let result = hasher.finalize();
    hex::encode(result)
}
