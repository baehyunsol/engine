let navbutton = document.getElementById("navbutton");
let topmenu = document.querySelector("div.topmenu");

function f() {
    topmenu.classList.toggle("activated");
    topmenu.classList.toggle("deactivated");
}

navbutton.onclick = f;