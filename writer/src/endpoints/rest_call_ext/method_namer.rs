use crate::{bail, report, Error, Result};
use convert_case::{Case, Casing};
use model::endpoint_docs::{HttpMethod, RestCall};

use super::RestCallExt;

fn remove_s(input: &str) -> &str {
    // If a string ends in 's' remove the 's'
    input.strip_suffix('s').unwrap_or(input)
}

struct Matcher<'a> {
    last_segment: &'a str,
    last_segment_is_param: bool,
    num_params: usize,
    last_segment_is_plural: bool,
    last_segment_is_endpoint: bool,
}

impl<'a> Matcher<'a> {
    fn new(call: &'a RestCall) -> Result<Matcher<'a>> {
        let is_param = |segment: &str| segment.starts_with('{') && segment.ends_with('}');
        let segments: Vec<&str> = call.path.split('/').collect();
        let last_segment = segments
            .iter()
            .cloned()
            .last()
            .ok_or_else(|| report!("Path with no segments: {call:#?}"))?;
        let last_segment_is_param = is_param(last_segment);
        let num_params = call
            .path
            .split('/')
            .filter(|segment| is_param(segment))
            .count();
        let last_segment_is_plural = last_segment.ends_with('s');
        let last_segment_is_endpoint = if last_segment_is_plural {
            remove_s(last_segment) == call.module_name()
        } else {
            last_segment == call.module_name()
        };
        Ok(Matcher {
            last_segment,
            last_segment_is_param,
            num_params,
            last_segment_is_plural,
            last_segment_is_endpoint,
        })
    }
}

/// Given a RestCall - generates a nice rust method name so we can generate code
pub fn method_name(call: &RestCall) -> Result<String> {
    let matcher = Matcher::new(call)?;
    Ok(match (&call.http_method, matcher) {
        (HttpMethod::Get, Matcher { num_params: 0, .. }) => "list_all".to_string(),
        (
            HttpMethod::Get,
            Matcher {
                num_params: 1,
                last_segment,
                last_segment_is_param: false,
                last_segment_is_endpoint: false,
                ..
            },
        ) => last_segment.to_case(Case::Snake),
        (
            HttpMethod::Get,
            Matcher {
                last_segment_is_param: true,
                last_segment_is_endpoint: false,
                ..
            },
        ) => "get".to_string(),
        (
            HttpMethod::Get,
            Matcher {
                num_params: 1,
                last_segment_is_endpoint: true,
                last_segment_is_plural: true,
                ..
            },
        ) => "list".to_string(),
        (
            HttpMethod::Patch,
            Matcher {
                num_params: 1,
                last_segment_is_param: false,
                last_segment_is_endpoint: true,
                ..
            },
        ) => "set".to_string(),
        (
            HttpMethod::Patch,
            Matcher {
                num_params: 1,
                last_segment,
                last_segment_is_param: false,
                ..
            },
        ) => format!("set_{last_segment}"),
        (
            HttpMethod::Post,
            Matcher {
                num_params: 1,
                last_segment_is_endpoint: true,
                ..
            },
        ) => "create".to_string(),
        (
            HttpMethod::Post,
            Matcher {
                num_params: 1,
                last_segment,
                last_segment_is_param: false,
                last_segment_is_endpoint: false,
                ..
            },
        ) => format!("create_{}", remove_s(&last_segment.to_case(Case::Snake))),
        (
            HttpMethod::Put,
            Matcher {
                last_segment_is_param: true,
                ..
            },
        ) => "set".to_string(),
        (
            HttpMethod::Put,
            Matcher {
                last_segment,
                last_segment_is_param: false,
                last_segment_is_endpoint: false,
                ..
            },
        ) => last_segment.to_case(Case::Snake),
        (
            HttpMethod::Put,
            Matcher {
                last_segment,
                last_segment_is_param: false,
                last_segment_is_endpoint: true,
                ..
            },
        ) => format!("change_{}", last_segment.to_case(Case::Snake)),

        _ => bail!("Unable to name RestCall: {call:#?}"),
    })
}

#[cfg(test)]
mod tests {
    use super::RestCallExt;
    use model::endpoint_docs::{Endpoint, HttpMethod, RestCall};
    use pretty_assertions::assert_eq;

    use crate::Result;

    #[test]
    fn test_method_name() -> Result<()> {
        // Expected rest calls and the method name that we should be generating
        let map: Vec<(RestCall, &str)> = vec![
            // Account endpoint
            (
                Endpoint::Account,
                HttpMethod::Get,
                "/v3/accounts",
                // Expected rust generated code method name
                "list_all",
            ),
            (
                Endpoint::Account,
                HttpMethod::Get,
                "/v3/accounts/{account_id}",
                "get",
            ),
            (
                Endpoint::Account,
                HttpMethod::Get,
                "/v3/accounts/{account_id}/summary",
                "summary",
            ),
            (
                Endpoint::Account,
                HttpMethod::Patch,
                "/v3/accounts/{account_id}/configuration",
                "set_configuration",
            ),
            // Instrument endpoint
            (
                Endpoint::Instrument,
                HttpMethod::Get,
                "/v3/instruments/{instrument}/candles",
                "candles",
            ),
            (
                Endpoint::Instrument,
                HttpMethod::Get,
                "/v3/instruments/{instrument}/orderBook",
                "order_book",
            ),
            // Order endpoint
            (
                Endpoint::Order,
                HttpMethod::Post,
                "/v3/accounts/{accountID}/orders",
                "create",
            ),
            (
                Endpoint::Order,
                HttpMethod::Get,
                "/v3/accounts/{accountID}/orders",
                "list",
            ),
            (
                Endpoint::Order,
                HttpMethod::Get,
                "/v3/accounts/{accountID}/pendingOrders",
                "pending_orders",
            ),
            (
                Endpoint::Order,
                HttpMethod::Get,
                "/v3/accounts/{accountID}/orders/{orderSpecifier}",
                "get",
            ),
            (
                Endpoint::Order,
                HttpMethod::Put,
                "/v3/accounts/{accountID}/orders/{orderSpecifier}",
                "set",
            ),
            (
                Endpoint::Order,
                HttpMethod::Put,
                "/v3/accounts/{accountID}/orders/{orderSpecifier}/cancel",
                "cancel",
            ),
            (
                Endpoint::Order,
                HttpMethod::Put,
                "/v3/accounts/{accountID}/orders/{orderSpecifier}/clientExtensions",
                "client_extensions",
            ),
            // Trade endpoint
            (
                Endpoint::Order,
                HttpMethod::Put,
                "/v3/accounts/{accountID}/trades/{tradeSpecifier}/orders",
                "change_orders",
            ),
            // Position endpoint
            (
                Endpoint::Position,
                HttpMethod::Get,
                "/v3/accounts/{accountID}/positions/{instrument}",
                "get",
            ),
            (
                Endpoint::Position,
                HttpMethod::Put,
                "/v3/accounts/{accountID}/positions/{instrument}/close",
                "close",
            ),
        ]
        .into_iter()
        .map(|(endpoint, http_method, path, method_name)| {
            (
                RestCall {
                    endpoint,
                    http_method,
                    path: path.to_string(),
                    ..Default::default()
                },
                method_name,
            )
        })
        .collect();
        for (rest_call, method_name) in map {
            assert_eq!(method_name, rest_call.method_name()?, "For {rest_call:#?}");
        }
        Ok(())
    }
}
