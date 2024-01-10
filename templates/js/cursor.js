let cursor_boxes = document.querySelectorAll("div.cursorbox pre");
let cursors = document.querySelectorAll("span.cursor");
let length = cursor_boxes.length;

for (let i = 0; i < length; i++) {
    cursor_boxes[i].addEventListener("mousemove", e => {
        let rect = cursor_boxes[i].getBoundingClientRect();

        cursors[i].style.left = e.clientX + 4 + "px";

        // these are seemingly constants, but they're not (font size change, scroll)
        // so they have to be calculated every frame
        cursors[i].style.height = (rect.bottom - rect.top) + "px";
        cursors[i].style.top = rect.y + window.scrollY + "px";
    });
}
