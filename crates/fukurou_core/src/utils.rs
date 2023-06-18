use std::time::Duration;

pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(60);

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);

// ///
// #[allow(clippy::result_large_err)]
// pub fn ureq_agent_builder(
//     proxy_url: Option<&str>,
// ) -> Result<ureq::AgentBuilder, ureq::Error> {
//     let builder = ureq::AgentBuilder::new()
//         .timeout_connect(DEFAULT_CONNECT_TIMEOUT)
//         .timeout_read(DEFAULT_TIMEOUT)
//         .timeout_write(DEFAULT_TIMEOUT);
//     if let Some(url) = proxy_url {
//         let proxy = ureq::Proxy::new(url)?;
//         Ok(builder.proxy(proxy))
//     } else {
//         Ok(builder)
//     }
// }
