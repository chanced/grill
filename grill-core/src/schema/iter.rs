//! Linear [`Iterator`]s of [`Schema`]s from [`Key`]s.
//!
use either::Either;
use slotmap::Key;

use super::Schemas;
use crate::{criterion::Criterion, error::UnknownKeyError, source::Sources, Schema};
