let topmenu = document.querySelector("div.topmenu");

function toggle_header() {
    topmenu.classList.toggle("activated");
    topmenu.classList.toggle("deactivated");
}

document.getElementById("navbutton").addEventListener("click", toggle_header);