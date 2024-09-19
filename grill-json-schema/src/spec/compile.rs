use std::error::Error as StdError;

use grill_core::Resolve;
use slotmap::Key;

use crate::{compile, JsonSchema};

use super::Specification;
