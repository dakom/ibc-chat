use cosmwasm_std::{Addr, Event};
use serde::de::DeserializeOwned;

/// Extension trait to add methods to native cosmwasm events
pub trait CosmwasmEventExt {
    // these are the only two that require implementation
    // everything else builds on these

    /// Does the event have the given attribute?
    fn has_attr(&self, key: &str) -> bool;

    /// Parse the value associated with the key, if it exists
    fn try_map_attr<B>(&self, key: &str, f: impl Fn(&str) -> B) -> Option<B>;

    /// Parse the value associated with the key as JSON, if it exists
    fn try_json_attr<B: DeserializeOwned>(&self, key: &str) -> anyhow::Result<Option<B>> {
        match self.try_map_attr(key, |s| serde_json::from_str(s)) {
            None => Ok(None),
            Some(x) => Ok(Some(x?)),
        }
    }

    /// Parse the value associated with the key as JSON
    fn json_attr<B: DeserializeOwned>(&self, key: &str) -> anyhow::Result<B> {
        self.map_attr_result(key, |s| {
            serde_json::from_str(s).map_err(anyhow::Error::from)
        })
    }

    /// Parse the value associated with the key as a u64
    fn u64_attr(&self, key: &str) -> anyhow::Result<u64> {
        self.map_attr_result(key, |s| s.parse().map_err(anyhow::Error::from))
    }

    /// Parse the value associated with the key as a u64, if it exists
    fn try_u64_attr(&self, key: &str) -> anyhow::Result<Option<u64>> {
        match self.try_map_attr(key, |s| s.parse()) {
            None => Ok(None),
            Some(x) => Ok(Some(x?)),
        }
    }

    /// Parse a string attribute
    fn string_attr(&self, key: &str) -> anyhow::Result<String> {
        self.map_attr_ok(key, |s| s.to_string())
    }

    /// Parse an address attribute without checking validity
    fn unchecked_addr_attr(&self, key: &str) -> anyhow::Result<Addr> {
        self.map_attr_ok(key, |s| Addr::unchecked(s))
    }

    /// Parse an optional address attribute without checking validity
    fn try_unchecked_addr_attr(&self, key: &str) -> anyhow::Result<Option<Addr>> {
        self.try_map_attr(key, |s| Ok(Addr::unchecked(s)))
            .transpose()
    }

    /// Require an attribute and apply a function to the raw string value
    fn map_attr_ok<B>(&self, key: &str, f: impl Fn(&str) -> B) -> anyhow::Result<B> {
        match self.try_map_attr(key, f) {
            Some(x) => Ok(x),
            None => Err(anyhow::anyhow!("no such key {}", key)),
        }
    }

    /// Require an attribute and try to parse its value with the given function
    fn map_attr_result<B>(
        &self,
        key: &str,
        f: impl Fn(&str) -> anyhow::Result<B>,
    ) -> anyhow::Result<B> {
        // just need to remove the one level of nesting for "no such key"
        self.map_attr_ok(key, f)?
    }
}

impl CosmwasmEventExt for Event {
    fn has_attr(&self, key: &str) -> bool {
        self.attributes.iter().any(|a| a.key == key)
    }
    fn try_map_attr<B>(&self, key: &str, f: impl Fn(&str) -> B) -> Option<B> {
        self.attributes.iter().find_map(|a| {
            if a.key == key {
                Some(f(a.value.as_str()))
            } else {
                None
            }
        })
    }
}
