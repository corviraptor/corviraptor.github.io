import lustre/attribute.{class}
import lustre/element
import lustre/element/html
import routing

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
            attribute.src("image/katylogo_diagonal_white.svg"),
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

pub fn body(route, contents) -> element.Element(a) {
  html.div([attribute.class("flex justify-center grow")], [
    html.div([attribute.class("background-gradient")], []),
    html.main([attribute.class("max-w-6xl grow shrink  m-4 shadow-2xl/90")], [
      html.nav([], [
        html.ul(
          [
            attribute.class("flex justify-stretch items-stretch p-0 m-0"),
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
          attribute.class(
            "grow m-0 py-8 px-10 bg-(--text-background-color) text-(--body-text-color) border-5 border-(--text-background-lighter) border-l-(--text-background-darker) border-b-(--text-background-darker)",
          ),
        ],
        contents,
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
      attribute.class(
        "bg-(--accent-main-color) flex basis-full grow shrink p-0 m-0",
      ),
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
            #("text-(--body-text-color) tab", !is_active),
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
                grow text-(--body-text-color) text-2xl p-3 font-bold font-(family-name:--header-font)
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
