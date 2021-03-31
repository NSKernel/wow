#[cfg(feature = "wasm")]
ow_wasm_action::pass_json!(func);

#[cfg(feature = "bin")]
ow_wasm_action::json_args!(func);

pub fn func(_json: serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {
    Ok(serde_json::json!({ "random": rand::random::<u64>() }))
}