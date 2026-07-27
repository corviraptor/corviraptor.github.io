import gleam/dict.{type Dict}
import gleam/http/response
import gleam/time/timestamp
import mork/document
import routing
import rsvp
import theme

pub type Post {
  Post(title: String, date: timestamp.Timestamp, summary: String, file: String)
}

pub type Model {
  Model(
    posts: Dict(String, Post),
    route: routing.Route,
    markdown_document: document.Document,
    theme_colors: theme.Theme,
  )
}

pub type Message {
  UserNavigatedTo(route: routing.Route)
  GotMarkdownAsset(
    markdown_result: Result(response.Response(String), rsvp.Error(String)),
  )
  FinishedLoadingMarkdownAsset
  GotBlogPosts(
    posts_result: Result(response.Response(String), rsvp.Error(String)),
  )
  UserUpdatedTheme(new_theme: theme.Theme)
}
