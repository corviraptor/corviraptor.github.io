import gleam/http/response
import gleam/int
import gleam/option
import gleam/result
import gleam/time/calendar
import gleam/time/timestamp
import lustre
import lustre/attribute
import lustre/element.{type Element}
import lustre/element/html
import maud
import maud/components as maud_components
import modem
import mork
import mork/document
import niji
import rsvp
import theme
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
    theme_colors: theme.ThemeColorSet,
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
    Model(
      route:,
      posts:,
      markdown_document: mork.parse("## Loading..."),
      theme_colors: theme.default_theme_color_set(),
    )

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
  html.div(
    [
      attribute.class("flex flex-col"),
      ..theme.theme_attributes(model.theme_colors)
    ],
    [
      site_components.header(),
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
          routing.Links -> view_links()
          routing.NotFound(_) -> view_not_found()
        },
      ),
    ],
  )
}

// VIEW PAGES ------------------------------------------------------------------

fn render_markdown(model: Model) {
  render_markdown_with_extra_components(model, fn(x) { x })
}

fn render_markdown_with_extra_components(
  model: Model,
  components: fn(maud_components.Components(a)) -> maud_components.Components(a),
) {
  maud.render_document(
    model.markdown_document,
    maud_components.default()
      |> maud_components.h1(fn(_attributes, _id, children) {
        html.h1([attribute.class("markdown-h1")], children)
      })
      |> components,
  )
}

fn view_index(model: Model) -> List(Element(message)) {
  render_markdown_with_extra_components(model, fn(components) {
    components
    |> maud_components.img(fn(_attributes, uri, alt_text, title) {
      html.img([
        attribute.class("inline inline-img"),
        attribute.alt(alt_text),
        attribute.src(uri),
        title
          |> option.map(fn(img_title) { attribute.title(img_title) })
          |> option.lazy_unwrap(attribute.none),
      ])
    })
  })
}

//link(routing.Blog, "read my ramblings ->"),

fn view_posts(model: Model) -> List(Element(message)) {
  let posts =
    model.posts
    |> dict.values
    |> list.sort(fn(a, b) { timestamp.compare(a.date, b.date) })
    |> list.map(fn(post) {
      html.div([attribute.class("flex mt-10 p-0 rounded-lg")], [
        html.a(
          [
            attribute.class("grow panel-button p-5"),
            routing.href(routing.PostById(post.file)),
          ],
          [
            html.h3(
              [
                attribute.class(
                  "text-xl text-(--accent-main-lighter) font-light",
                ),
              ],
              [html.text(post.title)],
            ),
            html.p([attribute.class("mt-1")], [html.text(post.summary)]),
          ],
        ),
      ])
    })

  case model.posts |> dict.keys() |> list.length() > 0 {
    True -> [title("Posts"), ..posts]
    False -> [
      title("Posts"),
      site_components.route_button(routing.Blog, "", "Load Blog Posts"),
    ]
  }
}

fn format_date(timestamp) {
  let #(calendar_date, _) =
    timestamp.to_calendar(timestamp, calendar.utc_offset)
  int.to_string(calendar_date.year)
  <> "-"
  <> int.to_string(calendar.month_to_int(calendar_date.month))
  <> "-"
  <> int.to_string(calendar_date.day)
}

fn view_post(model: Model, post_file_name: String) -> List(Element(message)) {
  case dict.get(model.posts, post_file_name) {
    Error(_) -> view_not_found()
    Ok(post) -> [
      html.article([], [
        title(post.title),
        leading(format_date(post.date) <> " | " <> post.summary),
        ..render_markdown(model)
      ]),
      site_components.route_button(routing.Blog, "󰁍", "Go back?"),
    ]
  }
}

fn view_portfolio() -> List(Element(message)) {
  [
    title("Not done"),
    paragraph("AAAA"),
  ]
}

fn view_links() -> List(Element(message)) {
  [
    html.h1([attribute.class("markdown-h1")], [html.text("Software")]),
    html.ul([attribute.class("flex gap-3 flex-wrap")], [
      site_components.link_button(
        "https://codeberg.org/corviraptor",
        "",
        "Codeberg",
      ),
      site_components.link_button(
        "https://github.com/corviraptor",
        "",
        "Github",
      ),
    ]),
    html.h1([attribute.class("markdown-h1")], [html.text("Other Stuff")]),
    html.ul([attribute.class("flex gap-3 flex-wrap")], [
      site_components.link_button(
        "https://corviraptor.bandcamp.com/",
        "",
        "Bandcamp",
      ),
      site_components.link_button(
        "https://www.twitch.tv/corviraptor",
        "",
        "Twitch",
      ),
      site_components.link_button(
        "https://www.youtube.com/@corviraptor",
        "",
        "YouTube",
      ),
      site_components.link_button("https://ko-fi.com/corviraptor", "", "Ko-Fi"),
      site_components.link_button(
        "https://bsky.app/profile/corviraptor.bsky.social",
        "",
        "Bluesky",
      ),
      site_components.link_button(
        "https://www.tumblr.com/blog/corviraptor",
        "",
        "Tumblr",
      ),
      site_components.link_button(
        "https://mastodon.gamedev.place/@corviraptor",
        "",
        "Mastodon",
      ),
    ]),
  ]
}

fn view_not_found() -> List(Element(message)) {
  [
    title("Not found"),
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
