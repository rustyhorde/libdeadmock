// Copyright (c) 2018 libdeadmock developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `libdeadmock` request/response mapping
use crate::config::{Request, Response};
use std::cmp::{Ord, Ordering};
use std::fmt;

/// `libdeadmock` mapping configuration
#[derive(
    Clone, Debug, Default, Deserialize, Eq, Getters, Hash, MutGetters, PartialEq, Serialize,
)]
pub struct Mapping {
    /// The priority of this mapping.  Lower takes priority over higher in the case of multiple matches.
    #[get = "pub"]
    priority: u8,
    /// The request matching configuration.
    #[get = "pub"]
    request: Request,
    /// The response configuration.
    #[get = "pub"]
    response: Response,
}

impl Ord for Mapping {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Mapping {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        writeln!(f);
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod test {
    use super::Mapping;
    use crate::config::request::test::{full_request, partial_request};
    use crate::config::response::test::{full_response, partial_response};
    use std::collections::BTreeMap;

    const EMPTY_MAPPING: &str = r#"{"priority":0,"request":{},"response":{}}"#;
    const PARTIAL_MAPPING: &str = r#"{"priority":10,"request":{"method":"GET","url":"http://a.url.com"},"response":{"status":200,"headers":[{"key":"Content-Type","value":"application/json"}],"proxy_base_url":"http://cdcproxy.kroger.com"}}"#;
    const FULL_MAPPING: &str = r#"{"priority":10,"request":{"method":"GET","url":"http://a.url.com","url_pattern":".*jasonozias.*","headers":[{"key":"Content-Type","value":"application/json"}]},"response":{"status":200,"headers":[{"key":"Content-Type","value":"application/json"}],"body_file_name":"test.json","proxy_base_url":"http://cdcproxy.kroger.com","additional_proxy_request_headers":[{"key":"Authorization","value":"Basic abcdef123"}]}}"#;
    const BAD_MAPPING_JSON: &str = r#"{"priority":"abc"}"#;

    fn partial_mapping() -> Mapping {
        let mut mapping = Mapping::default();
        mapping.priority = 10;
        mapping.request = partial_request();
        mapping.response = partial_response();
        mapping
    }

    fn full_mapping() -> Mapping {
        let mut mapping = partial_mapping();
        mapping.request = full_request();
        mapping.response = full_response();
        mapping
    }

    #[test]
    fn order() {
        let mut first = Mapping::default();
        first.priority = 5;

        let mut second = Mapping::default();
        second.priority = 1;

        let mut third = Mapping::default();
        third.priority = 3;

        let mut mappings = BTreeMap::new();
        mappings.insert(first, "first");
        mappings.insert(second, "second");
        mappings.insert(third, "third");

        let priorities: Vec<(u8, &str)> = mappings.iter().map(|(k, v)| (k.priority, *v)).collect();
        assert_eq!(priorities, vec![(1, "second"), (3, "third"), (5, "first")]);
    }

    #[test]
    fn serialize_empty_mapping() {
        if let Ok(serialized) = serde_json::to_string(&Mapping::default()) {
            assert_eq!(serialized, EMPTY_MAPPING);
        } else {
            assert!(false, "Serialization not expected to fail!");
        }
    }

    #[test]
    fn serialize_partial_mapping() {
        if let Ok(serialized) = serde_json::to_string(&partial_mapping()) {
            assert_eq!(serialized, PARTIAL_MAPPING);
        } else {
            assert!(false, "Serialization not expected to fail!");
        }
    }

    #[test]
    fn serialize_full_mapping() {
        if let Ok(serialized) = serde_json::to_string(&full_mapping()) {
            assert_eq!(serialized, FULL_MAPPING);
        } else {
            assert!(false, "Serialization not expected to fail!");
        }
    }

    #[test]
    fn deserialize_empty_mapping() {
        if let Ok(deserialized) = serde_json::from_str::<Mapping>(EMPTY_MAPPING) {
            assert_eq!(deserialized, Mapping::default());
        } else {
            assert!(
                false,
                "Expected deserialization of string into Mapping to succeed!"
            );
        }
    }

    #[test]
    fn deserialize_partial_mapping() {
        if let Ok(deserialized) = serde_json::from_str::<Mapping>(PARTIAL_MAPPING) {
            assert_eq!(deserialized, partial_mapping());
        } else {
            assert!(
                false,
                "Expected deserialization of string into Mapping to succeed!"
            );
        }
    }

    #[test]
    fn deserialize_full_mapping() {
        if let Ok(deserialized) = serde_json::from_str::<Mapping>(FULL_MAPPING) {
            assert_eq!(deserialized, full_mapping());
        } else {
            assert!(
                false,
                "Expected deserialization of string into Mapping to succeed!"
            );
        }
    }

    #[test]
    fn deserialize_bad_mapping() {
        assert!(
            serde_json::from_str::<Mapping>(BAD_MAPPING_JSON).is_err(),
            "Expected the deserialization to fail!"
        );
    }
}