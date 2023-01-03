let topmenu = document.querySelector("div.topmenu");

function toggle_header() {
    topmenu.classList.toggle("activated");
    topmenu.classList.toggle("deactivated");

    if (topmenu.style.maxHeight) {
        topmenu.style.maxHeight = null;
    }

    else {
        topmenu.style.maxHeight = topmenu.scrollHeight + "px";
    }

}

document.getElementById("navbutton").addEventListener("click", toggle_header);