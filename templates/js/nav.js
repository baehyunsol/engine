var settings = document.getElementById("settingsmenubg");

function open_settings() {
    settings.style.display = "flex";
}

function close_settings() {
    settings.style.display = "none";
}

document.getElementById("settingsopenbutton").addEventListener("click", open_settings);
document.getElementById("settingsclosebutton").addEventListener("click", close_settings);

var root_doc = document.querySelector(":root");
var horizontal_padding = 0;

function loose_padding() {
    horizontal_padding -= 24;

    if (horizontal_padding < 0) {
        horizontal_padding = 0;
    }

    root_doc.style.setProperty("--horiz-padding", horizontal_padding + "px");
}

function tight_padding() {
    horizontal_padding += 24;

    if (horizontal_padding > 288) {
        horizontal_padding = 288;
    }

    root_doc.style.setProperty("--horiz-padding", horizontal_padding + "px");
}

document.getElementById("tightpaddingbutton").addEventListener("click", tight_padding);
document.getElementById("loosepaddingbutton").addEventListener("click", loose_padding);