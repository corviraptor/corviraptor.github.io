import lustre/attribute.{class}
import lustre/element
import lustre/element/html
import routing

pub fn header() -> element.Element(a) {
  html.header(
    [
      class("p-4 bg-(--accent-main-color) text-white"),
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
              class("font-bold font-(family-name:--header-font)"),
              //attribute.style("margin-left", "0.25em"),
            ],
            [
              html.text("Kathrynne Winter"),
            ],
          ),
        ],
      ),
    ],
  )
}

pub fn body(route, contents) -> element.Element(a) {
  html.main([attribute.class("bg-red-300 max-w-6xl")], [
    html.nav([attribute.class("flex justify-center items-center my-16")], [
      html.ul([attribute.class("flex space-x-8")], [
        tab(current: route, to: routing.Index, label: "About"),
        tab(current: route, to: routing.Blog, label: "Blog"),
      ]),
    ]),
    html.div(
      [
        attribute.class(
          "my-16  mx-autopx-32 py-8 bg-(--text-background-color) text-(--body-text-color) border-5 border-(--text-background-lighter) border-l-(--text-background-darker) border-b-(--text-background-darker)",
        ),
      ],
      contents,
    ),
  ])
}

pub fn tab(
  to target: routing.Route,
  current current: routing.Route,
  label text: String,
) -> element.Element(message) {
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
      attribute.styles([#("display", "flex"), #("flex", "auto")]),
    ],
    [
      html.a([routing.href(target)], [
        html.div(
          [
            attribute.classes([
              #(
                "flex grow basis-full justify-center items-center bg-(--text-background-color) text-(--body-text-color) border-5 border-(--text-background-lighter) border-l-(--text-background-darker) border-b-(--text-background-darker)",
                True,
              ),
              #("text-purple-600", is_active),
            ]),
          ],
          [
            html.text(text),
          ],
        ),
      ]),
    ],
  )
}
