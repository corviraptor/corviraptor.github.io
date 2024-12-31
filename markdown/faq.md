# faq

// pronouns?

She/They/It!

// what's that font you always use?

[Iosevka Slab](https://github.com/be5invis/Iosevka), a free and open-source programming typeface. I originally found it while looking for nice-looking condensed monospace fonts to use in my text editor, but I quickly fell in love with it and use it all over the place now, even in places where proportional fonts are normally used. I originally used the standard version, but slab-serif has really grown on me.

I find it easy to read compared to many other fonts, but I don't have any empirical evidence that it improves accessibility — although it appears that [font legibility can be extremely subjective anyways](https://www.sciencedirect.com/science/article/pii/S0042698919301087#s0180) (which is why I put in the option to switch the fonts on this site if you want!).

I'm using a [web-hosted version of Iosevka Slab](https://github.com/iosevka-webfonts/iosevka-slab) for now. I run a variant of the font I compiled myself on my machine, but I don't want to deal with font sanitization right now.

// how'd you make this website?

I made this website with WebAssembly and [Yew](https://yew.rs/), mostly just as an excuse to learn Rust, HTML, and CSS. It's hosted on Github Pages (although the source repository is hosted on Codeberg), which is why I'm using a hash
router instead of a normal browser router. If you're wondering why it looks so blocky and chunky, I just think it looks cute.
