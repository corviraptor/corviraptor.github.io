import gleam/http/response
import gleam/option
import gleam/result
import gleam/time/calendar
import gleam/time/timestamp
import lustre
import lustre/attribute
import lustre/dev/query
import lustre/element.{type Element}
import lustre/element/html
import maud
import maud/components
import modem
import mork
import mork/document
import rsvp
import tom

//import simplifile

import gleam/dict.{type Dict}
import gleam/list
import lustre/effect.{type Effect}

import routing
import site_components

pub fn main() {
  let app = lustre.application(init, update, view)
  let assert Ok(_) = lustre.start(app, "#app", Nil)

  Nil
}

// MODEL -----------------------------------------------------------------------

type Post {
  Post(title: String, date: timestamp.Timestamp, summary: String, file: String)
}

type Model {
  Model(
    posts: Dict(String, Post),
    route: routing.Route,
    markdown_document: document.Document,
  )
}

fn init(_) -> #(Model, Effect(Message)) {
  // The server for a typical SPA will often serve the application to *any*
  // HTTP request, and let the app itself determine what to show. Modem stores
  // the first URL so we can parse it for the app's initial route.
  let initial_uri = modem.initial_uri()
  let route = case initial_uri {
    Ok(uri) -> routing.parse_route(uri)
    Error(_) -> routing.Index
  }

  let posts =
    []
    |> dict.from_list
  // TODO: there has to be a better way to initialize an empty dict

  let model =
    Model(route:, posts:, markdown_document: mork.parse("## Loading..."))

  let effect =
    effect.batch([
      // We need to initialise modem in order for it to intercept links. To do that
      // we pass in a function that takes the `Uri` of the link that was clicked and
      // turns it into a `Message`.
      modem.init(fn(uri) {
        uri
        |> routing.parse_route
        |> UserNavigatedTo
      }),
      case initial_uri {
        Ok(_) -> get_current_markdown(model)
        Error(_) -> effect.none()
      },
    ])

  #(model, effect)
}

// UPDATE ----------------------------------------------------------------------

type Message {
  UserNavigatedTo(route: routing.Route)
  GotMarkdownAsset(
    markdown_result: Result(response.Response(String), rsvp.Error(String)),
  )
  FinishedLoadingMarkdownAsset
  GotBlogPosts(
    posts_result: Result(response.Response(String), rsvp.Error(String)),
  )
}

fn get_current_markdown(model: Model) {
  case model.route {
    routing.Index -> get_markdown("about")
    routing.PostById(file:) ->
      get_post_markdown(model, file) |> result.unwrap(effect.none())
    _ -> effect.none()
  }
}

fn get_post_markdown(model: Model, file: String) {
  use post <- result.try(dict.get(model.posts, file))
  Ok(get_markdown("blog/" <> post.file))
}

fn get_markdown(file: String) {
  rsvp.get(
    "/markdown/" <> file <> ".md",
    rsvp.expect_ok_response(GotMarkdownAsset),
  )
}

fn update(model: Model, message: Message) -> #(Model, Effect(Message)) {
  case message {
    UserNavigatedTo(route:) -> #(Model(..model, route:), get_blog_index())
    GotMarkdownAsset(markdown_result) -> #(
      Model(
        ..model,
        markdown_document: mork.parse(case markdown_result {
          Ok(markdown_response) -> markdown_response.body
          Error(error) ->
            "Markdown file failed to load: "
            <> case error {
              rsvp.BadBody -> "Bad Body"
              rsvp.BadUrl(string) -> "Bad Url: " <> string
              rsvp.HttpError(_) -> "Http Error"
              rsvp.JsonError(_) -> "Json Error"
              rsvp.NetworkError -> "Network Error"
              rsvp.UnhandledResponse(_) -> "Unhandled Response"
            }
        }),
      ),
      effect.from(fn(dispatch) {
        dispatch(FinishedLoadingMarkdownAsset)

        Nil
      }),
    )
    GotBlogPosts(posts_result:) -> #(
      Model(..model, posts: {
        case posts_result {
          Ok(posts_response) ->
            result.unwrap(parse_blog_index(posts_response), [])
            |> dict.from_list
          Error(_) -> model.posts
        }
      }),
      get_current_markdown(model),
    )
    FinishedLoadingMarkdownAsset -> #(model, effect.none())
  }
}

fn parse_blog_index(posts_response: response.Response(String)) {
  use index_toml <- result.try(
    tom.parse(posts_response.body) |> result.replace_error(Nil),
  )

  use posts_list <- result.try(
    tom.get_array(index_toml, ["posts"])
    |> result.replace_error(Nil),
  )

  Ok(
    list.filter_map(posts_list, post_from_table)
    |> list.map(fn(x) { #(x.file, x) }),
  )
}

fn post_from_table(table_toml: tom.Toml) {
  use table <- result.try(tom.as_table(table_toml) |> result.replace_error(Nil))
  use file <- result.try(
    tom.get_string(table, ["file"]) |> result.replace_error(Nil),
  )
  use date <- result.try(parse_date_from_toml(table))
  let title =
    result.unwrap(tom.get_string(table, ["title"]), "Failed to load title")
  let summary =
    result.unwrap(tom.get_string(table, ["summary"]), "Failed to load summary")

  Ok(Post(title, date, summary, file))
}

fn parse_date_from_toml(table) {
  use date_string <- result.try(
    tom.get_string(table, ["date"]) |> result.replace_error(Nil),
  )
  timestamp.parse_rfc3339(date_string)
}

fn get_blog_index() {
  rsvp.get("/markdown/blog/index.toml", rsvp.expect_ok_response(GotBlogPosts))
}

// VIEW ------------------------------------------------------------------------

fn view(model: Model) -> Element(Message) {
  html.div([], [
    site_components.header(),
    html.nav([attribute.class("flex justify-between items-center my-16")], [
      html.h1([attribute.class("text-purple-600 font-medium text-xl")], [
        html.a([routing.href(routing.Index)], [html.text("My little Blog")]),
      ]),
      html.ul([attribute.class("flex space-x-8")], [
        view_header_link(current: model.route, to: routing.Blog, label: "Blog"),
        view_header_link(
          current: model.route,
          to: routing.Index,
          label: "About",
        ),
      ]),
    ]),
    site_components.body(
      model.route,
      // Just like we would show different HTML based on some other state in the
      // model, we can also pattern match on our Route value to show different
      // views based on the current page!
      case model.route {
        routing.Index -> view_index(model)
        routing.Blog -> view_posts(model)
        routing.PostById(post_file_name) -> view_post(model, post_file_name)
        routing.Portfolio -> view_portfolio()
        routing.NotFound(_) -> view_not_found()
      },
    ),
  ])
}

fn view_header_link(
  to target: routing.Route,
  current current: routing.Route,
  label text: String,
) -> Element(message) {
  let is_active = case current, target {
    routing.PostById(_), routing.Blog -> True
    _, _ -> current == target
  }

  html.li(
    [
      attribute.classes([
        #("border-transparent border-b-2 hover:border-purple-600", True),
        #("text-purple-600", is_active),
      ]),
    ],
    [html.a([routing.href(target)], [html.text(text)])],
  )
}

//mork.configure()
//      |> mork.strip_frontmatter(True)
//      |> mork.emojis(True)
//      |> mork.tables(True)
// VIEW PAGES ------------------------------------------------------------------

fn render_markdown(model: Model) {
  maud.render_document(
    model.markdown_document,
    components.default()
      |> components.h1(fn(_attributes, _id, children) {
        html.h1([attribute.class("markdown-h1")], children)
      }),
  )
}

fn view_index(model: Model) -> List(Element(message)) {
  render_markdown(model)
}

//link(routing.Blog, "read my ramblings ->"),

fn view_posts(model: Model) -> List(Element(message)) {
  let posts =
    model.posts
    |> dict.values
    |> list.sort(fn(a, b) { timestamp.compare(a.date, b.date) })
    |> list.map(fn(post) {
      html.article([attribute.class("mt-14")], [
        html.h3([attribute.class("text-xl text-purple-600 font-light")], [
          html.a(
            [
              attribute.class("hover:underline"),
              routing.href(routing.PostById(post.file)),
            ],
            [
              html.text(post.title),
            ],
          ),
        ]),
        html.p([attribute.class("mt-1")], [html.text(post.summary)]),
      ])
    })

  [title("Posts"), ..posts]
}

//..render_markdown(model)
fn view_post(model: Model, post_file_name: String) -> List(Element(message)) {
  case dict.get(model.posts, post_file_name) {
    Error(_) -> view_not_found()
    Ok(post) -> [
      html.article([], [
        title(post.title),
        leading(post.summary),
        ..render_markdown(model)
        //html.div([attribute.id("body-markdown-target")], render_markdown(model)),
      ]),
      html.p([attribute.class("mt-14")], [link(routing.Blog, "<- Go back?")]),
    ]
  }
}

fn view_portfolio() -> List(Element(message)) {
  [
    title("Not done"),
    paragraph("AAAA"),
  ]
}

fn view_not_found() -> List(Element(message)) {
  [
    title("Not found"),
    paragraph(
      "You glimpse into the void and see -- nothing?
       Well that was somewhat expected.",
    ),
  ]
}

// VIEW HELPERS ----------------------------------------------------------------

fn title(title: String) -> Element(message) {
  html.h2([attribute.class("text-3xl")], [
    html.text(title),
  ])
}

fn leading(text: String) -> Element(message) {
  html.p([attribute.class("mt-8 text-lg")], [html.text(text)])
}

fn paragraph(text: String) -> Element(message) {
  html.p([attribute.class("mt-14")], [html.text(text)])
}

/// In some frontend frameworks you might see special `<Link />` components that
/// are used to handle navigation logic. Using modem, we can just use normal HTML
/// `<a>` elements and pass in the `href` attribute. This means we have the option
/// of rendering our app as static HTML in the future!
///
fn link(target: routing.Route, title: String) -> Element(message) {
  html.a(
    [
      routing.href(target),
      attribute.class("text-purple-600 hover:underline cursor-pointer"),
    ],
    [html.text(title)],
  )
}
