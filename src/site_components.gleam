import gleam/int
import gleam/time/calendar
import gleam/time/timestamp
import lustre/attribute.{class}
import lustre/element
import lustre/element/html
import lustre/event
import model
import niji
import routing
import theme

pub fn header() -> element.Element(a) {
  html.header(
    [
      class("p-4 bg-(--accent-main-color) text-white flex"),
      attribute.styles([
        #("border-bottom", "5px solid var(--accent-main-darker)"),
        #("border-top", "5px solid var(--accent-main-lighter)"),
        #("padding", "0em 1em"),
      ]),
    ],
    [
      html.a(
        [
          routing.href(routing.Index),
          attribute.styles([
            #("display", "flex"),
            #("flex", "initial"),
            #("flex-direction", "row"),
            #("align-items", "center"),
            #("font-size", "25px"),
            #("gap", "0.5em"),
          ]),
        ],
        [
          html.img([
            attribute.src("/image/katylogo_diagonal_white.svg"),
            attribute.alt("Kathrynne's Logo"),
            attribute.style("max-height", "4em"),
          ]),
          html.h1(
            [
              class(
                "font-bold text-shadow-none font-(family-name:--header-font)",
              ),
              //attribute.style("margin-left", "0.25em"),
            ],
            [
              html.text("Kathrynne Winter"),
            ],
          ),
        ],
      ),
      html.div([attribute.class("grow")], []),
    ],
  )
}

fn get_bg_style(site_theme: theme.Theme) {
  case site_theme.opaque_background {
    True -> niji.to_css(niji.rgb_to_oklch(site_theme.text_background))
    False ->
      niji.to_css(niji.rgb_to_oklch(
        niji.Rgb(..site_theme.text_background, alpha: 0.4),
      ))
  }
}

pub fn body(route, contents, site_theme: theme.Theme, theme_event) {
  html.div([attribute.class("flex justify-center grow")], [
    html.div([attribute.class("background-gradient")], []),
    html.main([attribute.class("max-w-6xl grow shrink m-4 ")], [
      html.nav([], [
        html.ul(
          [
            attribute.class(
              "flex justify-stretch items-stretch p-0 m-0 ml-1 mr-1 bg-none",
            ),
          ],
          [
            tab(current: route, to: routing.Index, label: "About"),
            tab(current: route, to: routing.Blog, label: "Blog"),
            tab(current: route, to: routing.Links, label: "Links"),
          ],
        ),
      ]),
      html.div(
        [
          attribute.class("main-text grow m-0 py-8 px-10 "),
          attribute.style("background-color", get_bg_style(site_theme)),
        ],
        contents,
      ),
    ]),
    html.div([attribute.class("flex flex-col")], [
      settings_panel(site_theme, theme_event),
    ]),
  ])
}

pub fn settings_panel(site_theme: theme.Theme, theme_event) {
  html.div(
    [
      attribute.class(
        "flex flex-col mt-10 p-4 main-text rounded-xl gap-4 items-stretch ",
      ),
      attribute.style("background-color", get_bg_style(site_theme)),
    ],
    [
      switch(
        "opaque_background",
        "Opaque Text Background",
        site_theme.opaque_background,
        theme_event(fn(_) {
          theme.Theme(
            ..site_theme,
            opaque_background: !site_theme.opaque_background,
          )
        }),
      ),
      switch(
        "text_stroke",
        "Text Stroke",
        site_theme.text_stroke,
        theme_event(fn(_) {
          theme.Theme(..site_theme, text_stroke: !site_theme.text_stroke)
        }),
      ),
      button(
        model.UserUpdatedTheme(theme.reset_all_colors(site_theme)),
        "󰦛",
        "Reset All Colors",
        theme.all_colors_are_default(site_theme),
        True,
        "p-2 grow flex",
      ),
      color_picker("Accent Color", site_theme, theme.Accent),
      color_picker("Text Background Color", site_theme, theme.TextBackground),
      color_picker("Text Color", site_theme, theme.TextColor),
    ],
  )
}

pub fn switch(id, label, checked, event) {
  html.div([attribute.class("panel mt-1 p-2 flex flex-col items-center")], [
    html.label([attribute.for(id), attribute.class("label-text")], [
      html.text(label),
      html.div(
        [
          attribute.class("flex flex-row mt-2 mb-0 gap-1 items-center "),
        ],
        [
          html.div(
            [
              attribute.class(
                "h-[26px] w-[50px] relative box-content rounded-lg ring-4 ring-(--text-background-lighter) shadow-md shadow-black",
              ),
              attribute.class(case checked {
                True -> "inset-shadow-sm inset-shadow-black/50"
                False -> "inset-shadow-sm inset-shadow-black"
              }),
              attribute.style("background", case checked {
                True ->
                  "linear-gradient(0deg, var(--accent-main-darker) 0%, var(--accent-main-color) 10%, var(--accent-main-lighter) 80%, var(--accent-main-light) 100%)"
                False ->
                  "linear-gradient(0deg, var(--text-background-darker) 0%, var(--text-background-dark) 10%, var(--text-background-color) 80%, var(--text-background-dark) 100%)"
              }),
            ],
            [
              html.input([
                attribute.type_("checkbox"),
                attribute.id(id),
                attribute.checked(checked),
                attribute.class("opacity-0"),
                event.on_input(event),
              ]),
              html.span(
                [
                  attribute.class(
                    "bg-(--text-background-lightest) absolute h-[20px] w-[20px] left-[3px] top-[3px] rounded-md",
                  ),
                  attribute.style(
                    "border",
                    "4px outset var(--text-background-lighter)",
                  ),
                  case checked {
                    True -> attribute.style("left", "26px")
                    False -> attribute.none()
                  },
                ],
                [],
              ),
            ],
          ),
          html.div([attribute.class("grow")], []),
        ],
      ),
    ]),
  ])
}

pub fn color_picker(
  label,
  current_theme: theme.Theme,
  color_type: theme.ThemeColor,
) {
  let id = theme.get_id(color_type) <> "-color"

  let current_color = theme.get_current_theme_color(current_theme, color_type)

  let event = fn(x) {
    model.UserUpdatedTheme(theme.set_theme_color(current_theme, color_type, x))
  }

  let reset_event =
    model.UserUpdatedTheme(theme.reset_theme_color(current_theme, color_type))

  html.div([attribute.class("panel mt-1 flex flex-col items-center")], [
    html.label([attribute.for(id), attribute.class("label-text")], [
      html.div([attribute.class("m-2")], [html.text(label)]),
      html.div(
        [
          attribute.class(
            "flex flex-row mt-1 mb-0 p-[5px] gap-1 items-center bg-(--text-background-dark) rounded-bl-lg rounded-br-lg",
          ),
        ],
        [
          button(
            reset_event,
            "󰦛",
            "",
            theme.is_default(current_theme, color_type),
            False,
            "pl-1 pr-1.5",
          ),
          html.input([
            attribute.type_("color"),
            attribute.id(id),
            attribute.value(niji.rgb_to_hex(current_color)),
            event.on_input(event),
          ]),
          html.p([attribute.class("label-text")], [
            html.text(niji.rgb_to_hex(current_color)),
          ]),
        ],
      ),
    ]),
  ])
}

pub fn tab(
  current current: routing.Route,
  to target: routing.Route,
  label text: String,
) -> element.Element(message) {
  let is_active = case current, target {
    routing.PostById(_), routing.Blog -> True
    _, _ -> current == target
  }

  html.li(
    [
      attribute.class("flex basis-full grow shrink p-0 m-0"),
    ],
    [
      html.a(
        [
          attribute.classes([
            #(
              "
                grow text-2xl p-3 font-bold font-(family-name:--header-font)
                flex justify-center items-center
              ",
              True,
            ),
            #("text-(--text-color) tab", !is_active),
            #("text-(--accent-main-lighter) active-tab", is_active),
          ]),
          case is_active {
            False -> routing.href(target)
            True -> attribute.none()
          },
        ],
        [
          html.text(text),
        ],
      ),
    ],
  )
}

pub fn route_button(
  target route: routing.Route,
  icon icon: String,
  label text: String,
) -> element.Element(message) {
  href_button(target: routing.href(route), icon:, label: text)
}

pub fn link_button(
  target url: String,
  icon icon: String,
  label text: String,
) -> element.Element(message) {
  href_button(target: attribute.href(url), icon:, label: text)
}

pub fn href_button(
  target href: attribute.Attribute(message),
  icon icon: String,
  label text: String,
) -> element.Element(message) {
  html.li(
    [
      attribute.classes([
        #("panel-button flex p-0 m-0 rounded-xl shadow-lg/50", True),
      ]),
    ],
    [
      html.a(
        [
          attribute.classes([
            #(
              "
                grow text-(--text-color) text-2xl p-3 font-bold font-(family-name:--header-font)
                flex justify-center items-center gap-2
              ",
              True,
            ),
          ]),
          href,
        ],
        [
          html.div(
            [attribute.class("font-(family-name:--symbols-font) text-3xl")],
            [
              html.text(icon),
            ],
          ),
          html.text(text),
        ],
      ),
    ],
  )
}

pub fn button(
  event event,
  icon icon: String,
  label text: String,
  disabled disabled: Bool,
  grow grow: Bool,
  classes classes: String,
) {
  html.div(
    [
      attribute.class(
        "p-0 m-0 rounded-xl label-text "
        <> case grow {
          True -> "grow flex"
          False -> ""
        },
      ),
    ],
    [
      html.button(
        [
          event.on_click(event),
          attribute.disabled(disabled),
          attribute.class("flex flex-row panel-button " <> classes),
        ],
        [
          html.div(
            [attribute.class("font-(family-name:--symbols-font) text-3xl")],
            [
              html.text(icon),
            ],
          ),
          html.text(text),
        ],
      ),
    ],
  )
}

pub fn blog_post_header(post: model.Post) {
  html.div(
    [
      class("p-4 text-white flex items-center align-center panel-header grow"),
    ],
    [
      html.div([attribute.class("flex flex-col m-0")], [
        title(post.title),
        leading(post.summary),
      ]),
      html.div([attribute.class("grow")], []),
      leading(format_date(post.date)),
    ],
  )
}

pub fn title(title: String) -> element.Element(message) {
  html.h2([attribute.class("text-3xl m-0 p-0")], [
    html.text(title),
  ])
}

pub fn leading(text: String) -> element.Element(message) {
  html.p(
    [
      attribute.class("mtext-lg label-text m-0 pb-0"),
      attribute.styles([
        #("margin-bottom", "0"),
      ]),
    ],
    [html.text(text)],
  )
}

pub fn paragraph(text: String) -> element.Element(message) {
  html.p([attribute.class("mt-14")], [html.text(text)])
}

pub fn format_date(timestamp) {
  let #(calendar_date, _) =
    timestamp.to_calendar(timestamp, calendar.utc_offset)
  int.to_string(calendar_date.year)
  <> "-"
  <> int.to_string(calendar.month_to_int(calendar_date.month))
  <> "-"
  <> int.to_string(calendar_date.day)
}
