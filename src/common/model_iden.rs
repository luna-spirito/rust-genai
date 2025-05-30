use serde::{Deserialize, Serialize};

use crate::ModelName;
use crate::adapter::AdapterKind;

/// Holds the adapter kind and model name in an efficient, clonable way.
///
/// This struct represents the association between an adapter kind
/// and a model name, allowing for easy conversion and instantiation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelIden {
	/// The adapter kind.
	pub adapter_kind: AdapterKind,
	/// The model name.
	pub model_name: ModelName,
}

/// Contructor
impl ModelIden {
	/// Create a new `ModelIden` with the given adapter kind and model name.
	pub fn new(adapter_kind: AdapterKind, model_name: impl Into<ModelName>) -> Self {
		Self {
			adapter_kind,
			model_name: model_name.into(),
		}
	}
}

impl ModelIden {
	pub fn with_name_or_clone(&self, new_name: Option<String>) -> ModelIden {
		if let Some(new_name) = new_name {
			if *self.model_name != new_name {
				ModelIden {
					adapter_kind: self.adapter_kind,
					model_name: new_name.into(),
				}
			} else {
				self.clone()
			}
		} else {
			self.clone()
		}
	}
}

impl<T> From<(AdapterKind, T)> for ModelIden
where
	T: Into<ModelName>,
{
	fn from((adapter_kind, model_name): (AdapterKind, T)) -> Self {
		Self {
			adapter_kind,
			model_name: model_name.into(),
		}
	}
}
