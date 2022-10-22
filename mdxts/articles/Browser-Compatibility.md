---
date: [2022, 10, 22]
tags: [browser, web, html, comparison]
preview: Browser compatibility comparison
---

# Browser Compatibility Comparison

I always try my best to fit my blog into the HTML5 specification. But what if the browsers do not support the spec? I suddenly got worried about it and I decided to do the test by myself.

## Browsers

I must include 3 major browsers: firefox, chrome and safari. I only tested on PC, not mobile. Since each browser uses the same engine for both platforms, the compatibility issue between two must be minor. I didn't include the Edge browser because it uses the Chrome's engine. I excluded Samsung Internet, Brave and many other browsers for the same reason.

I also tested on minor *or* outdated browsers: IE, [Ladybird], [Servo] and [NetSurf]. They all use their own engine, all built from scratch.

- Firefox
  - Version 106.0 on Windows 10
  - tested on 2022/10/20
- Chrome
  - Version 106.0.5249.119 on Windows 10
  - tested on 2022/10/20
- Safari
  - Version 15.3 on MacOS 12.2.1
  - tested on 2022/10/22
- IE
  - Version 11.0.19041.1566 on Windows 10
  - tested on 2022/10/20, but that doesn't matter at all: IE's not maintained anymore
- Ladybird
  - Not tested yet
- NetSurf
  - Version 3.10 on Windows 10
  - tested on 2022/10/20
- Servo
  - Version ?? on Windows 10
  - tested on 2022/10/20

[Ladybird]: https://github.com/SerenityOS/ladybird
[Servo]: https://servo.org/
[NetSurf]: https://www.netsurf-browser.org/

## Result

| Checkpoint                                    | Firefox  | Chrome    | Safari  | IE         | Ladybird  | NetSurf    | Servo     |
|-----------------------------------------------|----------|-----------|---------|------------|-----------|------------|-----------|
| Smooth Scroll                                 | O        | O         | X       | X          | ?         | X          | X         |
| CSS Variables                                 | O        | O         | O       | X          | ?         | X          | O         |
| MathML (without help of JS)                   | O        | X         | O       | X          | ?         | X          | X         |
| copy_button of fenced code blocks [^cbf]      | O        | O         | O       | X          | ?         | X          | X         |
| Transparent background color                  | O        | O         | O       | X          | ?         | X          | O         |
| Changing theme [^ct]                          | O        | O         | O       | X          | ?         | X[^nse1]   | X[^sve1]  |
| Tasklist: Checked marker [^tlcm]              | O        | O         | O       | O[^iee1]   | ?         | X[^nse2]   | O         |
| Tasklist: Triangle marker [^tlcm]             | O        | O         | O       | O[^iee1]   | ?         | X[^nse2]   | O         |
| Background blur when viewing an image [^bb]   | O        | O         | X       | X          | ?         | X[^nse3]   | X         |
| Alignment of the Settings menu [^alsm]        | O        | O         | O       | O          | ?         | X[^nse4]   | X[^sve1]  |
| Collapsible tables with animations [^ctwa]    | O        | O         | O       | O          | ?         | X          | O         |
| Special Characters [^specialchars]            | O        | O         | O       | O          | ?         | ▲[^nse5]   | O         |
| CSS Media Query: Screen Orientation           | O        | O         | O       | O          | ?         | X          | X         |
| CSS Media Query: Print                        | O        | X[^che1]  | O       | O          | ?         | X[^nse6]   | X[^sve2]  |
| Rendering Korean alphabets (한글)             | O        | O         | O       | O          | ?         | O          | O         |
| Score                                         | 15/15    | 13/15     | 13/15   | 8/15       | ?/15      | 1.5/15     | 7/15      |

Currently, firefox is the only browser that passes all the tests. It's surprising that even the major browsers are not rendering a valid html5 page properly.

[^ct]: It uses JS' DOM API to control CSS variables. CSS `color` has a transition effect.

[^cbf]: It uses JS' clipboard API.

[^tlcm]: It only uses CSS to draw the marker.

[^bb]: It uses CSS: `backdrop-filter: blur(5px);`.

[^alsm]: It uses CSS `justify-content` and `align-items`.

[^ctwa]: It uses basic JS DOM API and CSS transitions.

[^iee1]: It renders the shape correctly, but I can't see it due to issues with colors.

[^nse1]: I failed to change the theme because I can't open the settings menu with it.

[^nse2]: I can't see the markers because the browser doesn't support CSS variables. I changed the colors with dev-tools on IE, but this browser doesn't have dev-tools.

[^nse3]: I can't check it because it doesn't open the image viewer. I guess it doesn't support JS at all.

[^nse4]: I can't open the settings menu.

[^specialchars]: All the characters in this [this page](MDxt-Character-Reference.html).

[^nse5]: [[char=cong]] is too small to read.

[^sve1]: I can't open the settings menu.

[^che1]: It crashes when I attempt to print the page.

[^nse6]: Print button doesn't work.

[^sve2]: There's no print function.