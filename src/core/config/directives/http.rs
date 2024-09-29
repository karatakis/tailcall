use serde::{Deserialize, Serialize};
use tailcall_macros::{DirectiveDefinition, InputDefinition};

use crate::core::config::{Encoding, KeyValue, URLQuery};
use crate::core::http::Method;
use crate::core::is_default;
use crate::core::json::JsonSchema;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    schemars::JsonSchema,
    DirectiveDefinition,
    InputDefinition,
)]
#[directive_definition(locations = "FieldDefinition, Object")]
#[serde(deny_unknown_fields)]
/// The @http operator indicates that a field or node is backed by a REST API.
///
/// For instance, if you add the @http operator to the `users` field of the
/// Query type with a path argument of `"/users"`, it signifies that the `users`
/// field is backed by a REST API. The path argument specifies the path of the
/// REST API. In this scenario, the GraphQL server will make a GET request to
/// the API endpoint specified when the `users` field is queried.
pub struct Http {
    #[serde(rename = "onRequest", default, skip_serializing_if = "is_default")]
    /// onRequest field in @http directive gives the ability to specify the
    /// request interception handler.
    pub on_request: Option<String>,

    #[serde(rename = "baseURL", default, skip_serializing_if = "is_default")]
    /// This refers to the base URL of the API. If not specified, the default
    /// base URL is the one specified in the `@upstream` operator.
    pub base_url: Option<String>,

    #[serde(default, skip_serializing_if = "is_default")]
    /// The body of the API call. It's used for methods like POST or PUT that
    /// send data to the server. You can pass it as a static object or use a
    /// Mustache template to substitute variables from the GraphQL variables.
    pub body: Option<String>,

    #[serde(default, skip_serializing_if = "is_default")]
    /// The `encoding` parameter specifies the encoding of the request body. It
    /// can be `ApplicationJson` or `ApplicationXWwwFormUrlEncoded`. @default
    /// `ApplicationJson`.
    pub encoding: Encoding,

    #[serde(rename = "batchKey", default, skip_serializing_if = "is_default")]
    /// The `batchKey` dictates the path Tailcall will follow to group the returned items from the batch request. For more details please refer out [n + 1 guide](https://tailcall.run/docs/guides/n+1#solving-using-batching).
    pub batch_key: Vec<String>,

    #[serde(default, skip_serializing_if = "is_default")]
    /// The `headers` parameter allows you to customize the headers of the HTTP
    /// request made by the `@http` operator. It is used by specifying a
    /// key-value map of header names and their values.
    pub headers: Vec<KeyValue>,

    #[serde(default, skip_serializing_if = "is_default")]
    /// Schema of the input of the API call. It is automatically inferred in
    /// most cases.
    pub input: Option<JsonSchema>,

    #[serde(default, skip_serializing_if = "is_default")]
    /// This refers to the HTTP method of the API call. Commonly used methods
    /// include `GET`, `POST`, `PUT`, `DELETE` etc. @default `GET`.
    pub method: Method,

    /// This refers to the API endpoint you're going to call. For instance `https://jsonplaceholder.typicode.com/users`.
    ///
    /// For dynamic segments in your API endpoint, use Mustache templates for
    /// variable substitution. For instance, to fetch a specific user, use
    /// `/users/{{args.id}}`.
    pub path: String,

    #[serde(default, skip_serializing_if = "is_default")]
    /// Schema of the output of the API call. It is automatically inferred in
    /// most cases.
    pub output: Option<JsonSchema>,

    #[serde(default, skip_serializing_if = "is_default")]
    /// This represents the query parameters of your API call. You can pass it
    /// as a static object or use Mustache template for dynamic parameters.
    /// These parameters will be added to the URL.
    /// NOTE: Query parameter order is critical for batching in Tailcall. The
    /// first parameter referencing a field in the current value using mustache
    /// syntax is automatically selected as the batching parameter.
    pub query: Vec<URLQuery>,
    #[serde(default, skip_serializing_if = "is_default")]
    /// Enables deduplication of IO operations to enhance performance.
    ///
    /// This flag prevents duplicate IO requests from being executed
    /// concurrently, reducing resource load. Caution: May lead to issues
    /// with APIs that expect unique results for identical inputs, such as
    /// nonce-based APIs.
    pub dedupe: Option<bool>,
}