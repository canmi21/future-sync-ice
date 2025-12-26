use std::collections::HashMap;

#[doc = " A per-connection, key-value storage space."]
#[doc = ""]
#[doc = " Keys are expected to be lowercase and dot-separated (e.g., \"conn.ip\")."]
#[doc = " All values are stored as strings."]
pub(crate) type KvStore = HashMap<String, String>;
