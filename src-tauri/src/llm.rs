use std::path::Path;

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel};
use llama_cpp_2::sampling::LlamaSampler;
use std::num::NonZeroU32;
use tauri::{path::BaseDirectory, Manager};

const MAX_TOKENS: i32 = 1024;
const MODEL_FILENAME: &str = "Qwen2.5-1.5B-Instruct-Q4_0.gguf";

pub struct LlmEngine {
    backend: LlamaBackend,
    model: LlamaModel,
}

unsafe impl Send for LlmEngine {}
unsafe impl Sync for LlmEngine {}

impl LlmEngine {
    pub fn new(model_path: &Path) -> Result<Self, String> {
        let backend =
            LlamaBackend::init().map_err(|e| format!("Failed to init llama backend: {}", e))?;
        let model_params = LlamaModelParams::default();
        let model =
            LlamaModel::load_from_file(&backend, model_path, &model_params).map_err(|e| {
                format!(
                    "Failed to load LLM from {}: {}",
                    model_path.display(),
                    e
                )
            })?;
        log::info!("LLM loaded from: {}", model_path.display());
        Ok(Self { backend, model })
    }

    pub fn rewrite_transcript(&self, input: &str, include_sql: bool) -> Result<String, String> {
        let prompt = build_prompt(input, include_sql);
        let tokens = self
            .model
            .str_to_token(&prompt, AddBos::Always)
            .map_err(|e| format!("Tokenization failed: {}", e))?;
        let n_prompt = tokens.len() as i32;
        let n_ctx = n_prompt + MAX_TOKENS;
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(NonZeroU32::new(n_ctx as u32));
        let mut ctx = self
            .model
            .new_context(&self.backend, ctx_params)
            .map_err(|e| format!("Failed to create LLM context: {}", e))?;
        let mut batch = LlamaBatch::new(512, 1);
        let last_idx = tokens.len() as i32 - 1;
        for (i, token) in (0_i32..).zip(tokens.into_iter()) {
            batch
                .add(token, i, &[0], i == last_idx)
                .map_err(|e| format!("Batch add failed: {}", e))?;
        }
        ctx.decode(&mut batch)
            .map_err(|e| format!("Initial decode failed: {}", e))?;
        let mut n_cur = batch.n_tokens();
        let mut decoder = encoding_rs::UTF_8.new_decoder();
        let mut sampler = LlamaSampler::greedy();
        let mut output = String::new();
        let limit = n_prompt + MAX_TOKENS;
        while n_cur <= limit {
            let token = sampler.sample(&ctx, batch.n_tokens() - 1);
            sampler.accept(token);
            if self.model.is_eog_token(token) {
                break;
            }
            let piece = self
                .model
                .token_to_piece(token, &mut decoder, true, None)
                .map_err(|e| format!("Token decode failed: {}", e))?;
            output.push_str(&piece);
            batch.clear();
            batch
                .add(token, n_cur, &[0], true)
                .map_err(|e| format!("Batch add failed: {}", e))?;
            n_cur += 1;
            ctx.decode(&mut batch)
                .map_err(|e| format!("Decode step failed: {}", e))?;
        }
        Ok(output.trim().to_string())
    }
}

fn build_prompt(input: &str, include_sql: bool) -> String {
    let sql_block = if include_sql {
        "\n- If the text contains SQL, format it as proper SQL code\n\
         - Variable names should always use underscores"
    } else {
        ""
    };
    format!(
"<|im_start|>system\n\
You are an English editor.\n\
Rewrite the given text with the following strict rules:\n\
- Fix grammar, punctuation, and clarity\n\
- Preserve EXACT meaning\n\
- Do NOT answer questions\n\
- Do NOT explain anything\n\
- Output must be clean and minimal (like Whisper transcription)\n\
- No extra commentary, prefixes, or suffixes\n\
- Use paragraphs or bullet points ONLY if necessary\n\
If rewriting is not possible, return the original text exactly.\n\
{sql_block}\n\
<|im_end|>\n\
<|im_start|>user\n\
{input}\n\
<|im_end|>\n\
<|im_start|>assistant\n\
")
}

pub fn resolve_bundled_llm_model(
    handle: &tauri::AppHandle,
) -> Result<std::path::PathBuf, String> {
    if let Ok(env_path) = std::env::var("LLM_MODEL_PATH") {
        let path = std::path::PathBuf::from(&env_path);
        if path.exists() {
            return Ok(path);
        }
    }
    let resource = format!("models/{}", MODEL_FILENAME);
    handle
        .path()
        .resolve(&resource, BaseDirectory::Resource)
        .map_err(|e| format!("Failed to resolve bundled LLM model path: {}", e))
}
