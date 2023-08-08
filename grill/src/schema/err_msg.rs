pub(super) const INVALID_KEY: &str = "Schema Key not found. 
    This is likely caused by there being multiple Interrogators using the same Key type.
    To avoid this fatal error, use a different Key type for each Interrogator. \
    To create a new Key type, see the macro new_key_type, re-exported from slotmap.

    If that is not the issue, please open a ticket at https://github.com/chanced/grill/issues/new";
