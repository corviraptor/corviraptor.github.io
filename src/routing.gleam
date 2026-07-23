// IMPORTS ---------------------------------------------------------------------

import gleam/int
import gleam/uri.{type Uri}
import lustre/attribute.{type Attribute}

/// In a real application, we'll likely want to show different views depending on
/// which URL we are on:
///
/// - /      - show the home page
/// - /posts - show a list of posts
/// - /about - show an about page
/// - ...
///
/// We could store the `Uri` or perhaps the path as a `String` in our model, but
/// this can be awkward to work with and error prone as our application grows.
///
/// Instead, we _parse_ the URL into a nice Gleam custom type with just the
/// variants we need! This lets us benefit from Gleam's pattern matching,
/// exhaustiveness checks, and LSP features, while also serving as documentation
/// for our app: if you can get to a page, it must be in this type!
///
pub type Route {
  Index
  Blog
  PostById(file: String)
  Portfolio
  /// It's good practice to store whatever `Uri` we failed to match in case we
  /// want to log it or hint to the user that maybe they made a typo.
  NotFound(uri: Uri)
}

pub fn parse_route(uri: Uri) -> Route {
  case uri.path_segments(uri.path) {
    [] | [""] -> Index

    ["blog"] -> Blog

    ["post", post_file_name] -> PostById(file: post_file_name)
    ["portfolio"] -> Portfolio

    _ -> NotFound(uri:)
  }
}

/// We also need a way to turn a Route back into a an `href` attribute that we
/// can then use on `html.a` elements. It is important to keep this function in
/// sync with the parsing, but once you do, all links are guaranteed to work!
///
pub fn href(route: Route) -> Attribute(message) {
  let url = case route {
    Index -> "/"
    Blog -> "/blog"
    PostById(post_file_name) -> "/post/" <> post_file_name
    Portfolio -> "/portfolio"
    NotFound(_) -> "/404"
  }

  attribute.href(url)
}
