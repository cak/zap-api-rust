/* Zed Attack Proxy (ZAP) and its related class files.
 *
 * ZAP is an HTTP/HTTPS proxy for assessing web application security.
 *
 * Copyright 2019 the ZAP development team
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::ZapApiError;
use super::ZapService;
use serde_json::Value;
use std::collections::HashMap;

/**
 * This file was automatically generated.
 */
/**
 * Returns the URLs of the HTTP messages that match the given regular expression in the URL optionally filtered by URL and paginated with 'start' position and 'count' of messages.
*/
pub fn urls_by_url_regex(
    service: &ZapService,
    regex: String,
    baseurl: String,
    start: String,
    count: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "search", "view", "urlsByUrlRegex", params)
}

/**
 * Returns the URLs of the HTTP messages that match the given regular expression in the request optionally filtered by URL and paginated with 'start' position and 'count' of messages.
*/
pub fn urls_by_request_regex(
    service: &ZapService,
    regex: String,
    baseurl: String,
    start: String,
    count: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "search", "view", "urlsByRequestRegex", params)
}

/**
 * Returns the URLs of the HTTP messages that match the given regular expression in the response optionally filtered by URL and paginated with 'start' position and 'count' of messages.
*/
pub fn urls_by_response_regex(
    service: &ZapService,
    regex: String,
    baseurl: String,
    start: String,
    count: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "search", "view", "urlsByResponseRegex", params)
}

/**
 * Returns the URLs of the HTTP messages that match the given regular expression in the header(s) optionally filtered by URL and paginated with 'start' position and 'count' of messages.
*/
pub fn urls_by_header_regex(
    service: &ZapService,
    regex: String,
    baseurl: String,
    start: String,
    count: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "search", "view", "urlsByHeaderRegex", params)
}

/**
 * Returns the HTTP messages that match the given regular expression in the URL optionally filtered by URL and paginated with 'start' position and 'count' of messages.
*/
pub fn messages_by_url_regex(
    service: &ZapService,
    regex: String,
    baseurl: String,
    start: String,
    count: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "search", "view", "messagesByUrlRegex", params)
}

/**
 * Returns the HTTP messages that match the given regular expression in the request optionally filtered by URL and paginated with 'start' position and 'count' of messages.
*/
pub fn messages_by_request_regex(
    service: &ZapService,
    regex: String,
    baseurl: String,
    start: String,
    count: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "search", "view", "messagesByRequestRegex", params)
}

/**
 * Returns the HTTP messages that match the given regular expression in the response optionally filtered by URL and paginated with 'start' position and 'count' of messages.
*/
pub fn messages_by_response_regex(
    service: &ZapService,
    regex: String,
    baseurl: String,
    start: String,
    count: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "search", "view", "messagesByResponseRegex", params)
}

/**
 * Returns the HTTP messages that match the given regular expression in the header(s) optionally filtered by URL and paginated with 'start' position and 'count' of messages.
*/
pub fn messages_by_header_regex(
    service: &ZapService,
    regex: String,
    baseurl: String,
    start: String,
    count: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "search", "view", "messagesByHeaderRegex", params)
}

/**
 * Returns the HTTP messages, in HAR format, that match the given regular expression in the URL optionally filtered by URL and paginated with 'start' position and 'count' of messages.
*/
pub fn har_by_url_regex(
    service: &ZapService,
    regex: String,
    baseurl: String,
    start: String,
    count: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "search", "other", "harByUrlRegex", params)
}

/**
 * Returns the HTTP messages, in HAR format, that match the given regular expression in the request optionally filtered by URL and paginated with 'start' position and 'count' of messages.
*/
pub fn har_by_request_regex(
    service: &ZapService,
    regex: String,
    baseurl: String,
    start: String,
    count: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "search", "other", "harByRequestRegex", params)
}

/**
 * Returns the HTTP messages, in HAR format, that match the given regular expression in the response optionally filtered by URL and paginated with 'start' position and 'count' of messages.
*/
pub fn har_by_response_regex(
    service: &ZapService,
    regex: String,
    baseurl: String,
    start: String,
    count: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "search", "other", "harByResponseRegex", params)
}

/**
 * Returns the HTTP messages, in HAR format, that match the given regular expression in the header(s) optionally filtered by URL and paginated with 'start' position and 'count' of messages.
*/
pub fn har_by_header_regex(
    service: &ZapService,
    regex: String,
    baseurl: String,
    start: String,
    count: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "search", "other", "harByHeaderRegex", params)
}
