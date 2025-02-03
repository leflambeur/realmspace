use anyhow::{Error as E, Result};
use burn::prelude::*;
use candle_core::quantized::gguf_file::Content;
use candle_core::Device;
use llama_burn::llama::{LlamaConfig, RopeConfig};

fn format_size(size_in_bytes: usize) -> String {
	if size_in_bytes < 1_000 {
		format!("{}B", size_in_bytes)
	} else if size_in_bytes < 1_000_000 {
		format!("{:.2}KB", size_in_bytes as f64 / 1e3)
	} else if size_in_bytes < 1_000_000_000 {
		format!("{:.2}MB", size_in_bytes as f64 / 1e6)
	} else {
		format!("{:.2}GB", size_in_bytes as f64 / 1e9)
	}
}

fn main() -> Result<()> {
	let path = "../../models/Rocinante/Rocinante-12B-v1.1-Q8_0.gguf";
	let mut file = std::fs::File::open(path).map_err(E::msg)?;
	let content = Content::read(&mut file).map_err(|e| e.with_path(path))?;
	let mut total_size_in_bytes = 0;
	for (_, tensor) in content.tensor_infos.iter() {
		let elem_count = tensor.shape.elem_count();
		total_size_in_bytes +=
			elem_count * tensor.ggml_dtype.type_size() / tensor.ggml_dtype.block_size();
	}
	println!(
		"Loaded {:?} tensors ({})",
		content.tensor_infos.len(),
		&format_size(total_size_in_bytes),
	);
	let device = Device::new_metal(0)?;
	for (key, value) in content.metadata.iter() {
		if let Some(stripped) = key.strip_prefix("general.") {
			print!("general.{}: {:?}", stripped, value);
		}
		if let Some(stripped) = key.strip_prefix("llama.") {
			print!("llama.{}: {:?}", stripped, value);
		}
	}
	let architecture =
		&content.metadata.get("general.architecture").unwrap().to_string()?.to_owned();
	let burn_config = LlamaConfig::new(
		content.metadata.get("llama.feed_forward_length").unwrap().to_u32()?.to_owned() as usize,
		content.metadata.get("llama.vocab_size").unwrap().to_u32()?.to_owned() as usize,
		"../../models/Rocinante/tokenizer.json".to_owned(),
	)
	.with_num_attention_heads(
		content.metadata.get("llama.attention.head_count").unwrap().to_u32()?.to_owned() as usize,
	)
	.with_num_key_value_heads(Some(
		content.metadata.get("llama.attention.head_count_kv").unwrap().to_u32()?.to_owned()
			as usize,
	))
	.with_rope(RopeConfig::new(
		content.metadata.get("llama.rope.freq_base").unwrap().to_f32()?.to_owned(),
	))
	.with_norm_eps(
		content
			.metadata
			.get("llama.attention.layer_norm_rms_epsilon")
			.unwrap()
			.to_f64()?
			.to_owned(),
	)
	.with_num_hidden_layers(
		content.metadata.get("llama.block_count").unwrap().to_u32()?.to_owned() as usize,
	);

	println!("{:#?}", content.tensor_infos);
	Ok(())
}
