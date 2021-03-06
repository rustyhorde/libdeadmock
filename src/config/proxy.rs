// Copyright (c) 2018 libdeadmock developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `libdeadmock` proxy configuration
use crate::error::Error::{self, InvalidProxyConfig};
use clap::ArgMatches;
use getset::{Getters, Setters};
use std::convert::TryFrom;

/// The proxy configuration for deadmock
///
/// # Example
///
/// ## With `clap` [`ArgMatches`](clap::ArgMatches)
/// ```
/// # #![feature(try_from)]
/// # use clap::{App, Arg};
/// # use libdeadmock::config;
/// # use std::error::Error;
/// # use std::convert::TryFrom;
/// #
/// # fn test_cli() -> App<'static, 'static> {
/// #     App::new("proxy-config-test")
/// #         .version("1")
/// #         .author("Yoda")
/// #         .about("command line for proxy config testing")
/// #         .arg(
/// #             Arg::with_name("proxy")
/// #                 .short("p")
/// #                 .long("proxy")
/// #                 .requires("proxy-url")
/// #                 .help("Use a proxy"),
/// #         ).arg(
/// #             Arg::with_name("proxy-url")
/// #                 .long("proxy-url")
/// #                 .takes_value(true)
/// #                 .value_name("PROXY_URL")
/// #                 .help("Your proxy url, if applicable"),
/// #         ).arg(
/// #             Arg::with_name("proxy-username")
/// #                 .long("proxy-username")
/// #                 .takes_value(true)
/// #                 .value_name("PROXY_USER")
/// #                 .help("Your proxy username, if applicable"),
/// #         ).arg(
/// #             Arg::with_name("proxy-password")
/// #                 .long("proxy-password")
/// #                 .takes_value(true)
/// #                 .value_name("PROXY_PASS")
/// #                 .help("Your proxy password, if applicable"),
/// #         )
/// # }
/// #
/// # fn proxy_config() -> Result<(), Error> {
///     let arg_vec = vec!["test-cli", "-p", "--proxy-url", "http://a.proxy.com"];
///     let matches = test_cli().get_matches_from_safe(arg_vec)?;
///     let proxy_config = config::Proxy::try_from(&matches)?;
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #    proxy_config().unwrap();
/// # }
/// ```
///
/// ## Minimal
/// ```
/// # use libdeadmock::config;
/// #
/// # fn main() {
///     // When the proxy is disabled.
///     let disabled_proxy = config::Proxy::default();
///
///     // When using a proxy.
///     let proxy_config = config::Proxy::new(true, Some("http://a.proxyurl.com".to_string()));
/// # }
/// ```
#[derive(Clone, Debug, Default, Getters, Hash, Eq, PartialEq, Setters)]
pub struct Proxy {
    /// Turn the proxy on.  If this is true, `proxy_url` is required.
    #[get = "pub"]
    #[set = "pub"]
    use_proxy: bool,
    /// The proxy url.
    #[get = "pub"]
    #[set = "pub"]
    proxy_url: Option<String>,
    /// Username for proxy authentication.
    #[get = "pub"]
    #[set = "pub"]
    proxy_username: Option<String>,
    /// Password for proxy authentication.
    #[get = "pub"]
    #[set = "pub"]
    proxy_password: Option<String>,
}

impl Proxy {
    /// Create a new minimal proxy configuration.
    ///
    /// # Example
    /// ```
    /// # use libdeadmock::config;
    /// #
    /// # fn main() {
    ///     // When the proxy is disabled.
    ///     let disabled_proxy = config::Proxy::default();
    ///
    ///     // When using a proxy.
    ///     let proxy_config = config::Proxy::new(true, Some("http://a.proxyurl.com".to_string()));
    /// # }
    /// ```
    pub fn new(use_proxy: bool, proxy_url: Option<String>) -> Self {
        Self {
            use_proxy,
            proxy_url,
            proxy_username: None,
            proxy_password: None,
        }
    }
}

impl<'a> TryFrom<&'a ArgMatches<'a>> for Proxy {
    type Error = Error;

    fn try_from(matches: &'a ArgMatches<'a>) -> Result<Self, Error> {
        let use_proxy = matches.is_present("proxy");
        let proxy_url = matches.value_of("proxy-url").map(|s| s.to_string());
        let proxy_username = matches.value_of("proxy-username").map(|s| s.to_string());
        let proxy_password = matches.value_of("proxy-password").map(|s| s.to_string());

        if use_proxy && proxy_url.is_some() {
            Ok(Self {
                proxy_url,
                use_proxy,
                proxy_username,
                proxy_password,
            })
        } else if use_proxy && proxy_url.is_none() {
            Err(InvalidProxyConfig)
        } else {
            Ok(Self {
                proxy_url,
                use_proxy,
                proxy_username,
                proxy_password,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::Proxy;
    use clap::{App, Arg};
    use std::convert::TryFrom;

    fn test_cli() -> App<'static, 'static> {
        App::new("proxy-config-test")
            .version("1")
            .author("Yoda")
            .about("command line for proxy config testing")
            .arg(
                Arg::with_name("proxy")
                    .short("p")
                    .long("proxy")
                    .requires("proxy-url")
                    .help("Use a proxy"),
            )
            .arg(
                Arg::with_name("proxy-url")
                    .long("proxy-url")
                    .takes_value(true)
                    .value_name("PROXY_URL")
                    .help("Your proxy url, if applicable"),
            )
            .arg(
                Arg::with_name("proxy-username")
                    .long("proxy-username")
                    .takes_value(true)
                    .value_name("PROXY_USER")
                    .help("Your proxy username, if applicable"),
            )
            .arg(
                Arg::with_name("proxy-password")
                    .long("proxy-password")
                    .takes_value(true)
                    .value_name("PROXY_PASS")
                    .help("Your proxy password, if applicable"),
            )
    }

    fn test_cli_no_requires() -> App<'static, 'static> {
        App::new("proxy-config-test")
            .version("1")
            .author("Yoda")
            .about("command line for proxy config testing")
            .arg(
                Arg::with_name("proxy")
                    .short("p")
                    .long("proxy")
                    .help("Use a proxy"),
            )
            .arg(
                Arg::with_name("proxy-url")
                    .long("proxy-url")
                    .takes_value(true)
                    .value_name("PROXY_URL")
                    .help("Your proxy url, if applicable"),
            )
    }

    #[test]
    fn default_is_disabled() {
        let proxy_config = Proxy::default();
        assert!(!proxy_config.use_proxy());
    }

    #[test]
    fn from_all_args() {
        let arg_vec = vec![
            "test-cli",
            "-p",
            "--proxy-url",
            "http://a.proxy.com",
            "--proxy-username",
            "test",
            "--proxy-password",
            "test",
        ];
        let matches = test_cli().get_matches_from(arg_vec);
        match Proxy::try_from(&matches) {
            Ok(proxy_config) => {
                assert!(proxy_config.use_proxy());
                assert_eq!(
                    proxy_config.proxy_url(),
                    &Some("http://a.proxy.com".to_string())
                );
                assert_eq!(proxy_config.proxy_username(), &Some("test".to_string()));
                assert_eq!(proxy_config.proxy_password(), &Some("test".to_string()));
            }
            Err(_) => assert!(false, "Not expected to error!"),
        }
    }

    #[test]
    fn no_username_password() {
        let arg_vec = vec!["test-cli", "-p", "--proxy-url", "http://a.proxy.com"];
        let matches = test_cli().get_matches_from(arg_vec);
        match Proxy::try_from(&matches) {
            Ok(proxy_config) => {
                assert!(proxy_config.use_proxy());
                assert_eq!(
                    proxy_config.proxy_url(),
                    &Some("http://a.proxy.com".to_string())
                );
                assert!(proxy_config.proxy_username().is_none());
                assert!(proxy_config.proxy_password().is_none());
            }
            Err(_) => assert!(false, "Not expected to error!"),
        }
    }

    #[test]
    fn proxy_requires_proxy_url() {
        let arg_vec = vec!["test-cli", "-p", "--proxy-username", "password"];
        assert!(test_cli().get_matches_from_safe(arg_vec).is_err());
    }

    #[test]
    fn proxy_config_requires_proxy_url() {
        let arg_vec = vec!["test-cli", "-p"];
        let matches = test_cli_no_requires().get_matches_from(arg_vec);
        match Proxy::try_from(&matches) {
            Ok(_) => assert!(false, "Not expected to succeed!"),
            Err(e) => assert_eq!(format!("{}", e), "invalid proxy configuration!"),
        }
    }
}
