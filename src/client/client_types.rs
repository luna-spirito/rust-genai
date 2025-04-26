use crate::ClientBuilder;
use crate::chat::ChatMessage;
use crate::client::ClientConfig;
use crate::webc::WebClient;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

/// genai Client for executing AI requests to any providers.
/// Built with:
/// - `ClientBuilder::default()...build()`
/// - or `Client::builder()`, which is equivalent to `ClientBuilder::default()...build()`
#[derive(Debug, Clone)]
pub struct Client {
	pub(super) inner: Arc<ClientInner>,
}

// region:    --- Client Constructors

impl Default for Client {
	fn default() -> Self {
		Client::builder().build()
	}
}

impl Client {
	/// Create a new ClientBuilder for Client
	/// This is just another way to use `ClientBuilder::default()`
	pub fn builder() -> ClientBuilder {
		ClientBuilder::default()
	}
}

// endregion: --- Client Constructors

// region:    --- Client Getters

impl Client {
	pub(crate) fn web_client(&self) -> &WebClient {
		&self.inner.web_client
	}

	pub(crate) fn config(&self) -> &ClientConfig {
		&self.inner.config
	}

	pub(crate) fn cache(&self) -> &ClientCache {
		&self.inner.cache
	}
}

// endregion: --- Client Getters

// region:    --- ClientInner

#[derive(Debug)]
pub(super) struct ClientInner {
	pub(super) web_client: WebClient,

	pub(super) config: ClientConfig,

	pub(super) cache: ClientCache,
}

// endregion: --- ClientInner

// region:    --- GeminiCacheEntry

pub(crate) type ClientCache = RwLock<Vec<GeminiCacheEntry>>; // Just Gemini cache for now

#[derive(Debug)]
pub(crate) struct GeminiCacheEntry {
	pub(crate) endpoint_base_url: String,
	pub(crate) api_key: String, // Cache is local to API key?

	pub(crate) name: String,
	pub(crate) contents: Vec<ChatMessage>,
	pub(crate) expires_at: SystemTime,
}

// endregion: --- GeminiCacheEntry
