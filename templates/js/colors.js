var root_doc = document.querySelector(":root");
var is_dark = true;

function change_theme() {

    if (is_dark) {
        to_light();
        change_theme_button.innerHTML = "Set Dark Theme";
    }

    else {
        to_dark();
        change_theme_button.innerHTML = "Set Light Theme";
    }

    is_dark = !is_dark;
}

function to_dark() {
    
    root_doc.style.setProperty("--black", "#000000");
    root_doc.style.setProperty("--black-compl", "#ffffff");
    root_doc.style.setProperty("--black-trans", "#00000080");
    root_doc.style.setProperty("--black-compl-trans", "#ffffff80");
    
    root_doc.style.setProperty("--dark", "#404040");
    root_doc.style.setProperty("--dark-compl", "#bfbfbf");
    root_doc.style.setProperty("--dark-trans", "#40404080");
    root_doc.style.setProperty("--dark-compl-trans", "#bfbfbf80");
    
    root_doc.style.setProperty("--gray", "#808080");
    root_doc.style.setProperty("--gray-compl", "#7f7f7f");
    root_doc.style.setProperty("--gray-trans", "#80808080");
    root_doc.style.setProperty("--gray-compl-trans", "#7f7f7f80");
    
    root_doc.style.setProperty("--lightgray", "#c0c0c0");
    root_doc.style.setProperty("--lightgray-compl", "#3f3f3f");
    root_doc.style.setProperty("--lightgray-trans", "#c0c0c080");
    root_doc.style.setProperty("--lightgray-compl-trans", "#3f3f3f80");
    
    root_doc.style.setProperty("--white", "#ffffff");
    root_doc.style.setProperty("--white-compl", "#000000");
    root_doc.style.setProperty("--white-trans", "#ffffff80");
    root_doc.style.setProperty("--white-compl-trans", "#00000080");
    
    root_doc.style.setProperty("--red", "#c04040");
    root_doc.style.setProperty("--red-compl", "#3fbfbf");
    root_doc.style.setProperty("--red-trans", "#c0404080");
    root_doc.style.setProperty("--red-compl-trans", "#3fbfbf80");
    
    root_doc.style.setProperty("--green", "#40c040");
    root_doc.style.setProperty("--green-compl", "#bf3fbf");
    root_doc.style.setProperty("--green-trans", "#40c04080");
    root_doc.style.setProperty("--green-compl-trans", "#bf3fbf80");
    
    root_doc.style.setProperty("--blue", "#4040c0");
    root_doc.style.setProperty("--blue-compl", "#bfbf3f");
    root_doc.style.setProperty("--blue-trans", "#4040c080");
    root_doc.style.setProperty("--blue-compl-trans", "#bfbf3f80");
    
    root_doc.style.setProperty("--aqua", "#40c0ff");
    root_doc.style.setProperty("--aqua-compl", "#bf3f00");
    root_doc.style.setProperty("--aqua-trans", "#40c0ff80");
    root_doc.style.setProperty("--aqua-compl-trans", "#bf3f0080");
    
    root_doc.style.setProperty("--emerald", "#40ffc0");
    root_doc.style.setProperty("--emerald-compl", "#bf003f");
    root_doc.style.setProperty("--emerald-trans", "#40ffc080");
    root_doc.style.setProperty("--emerald-compl-trans", "#bf003f80");
    
    root_doc.style.setProperty("--violet", "#c040ff");
    root_doc.style.setProperty("--violet-compl", "#3fbf00");
    root_doc.style.setProperty("--violet-trans", "#c040ff80");
    root_doc.style.setProperty("--violet-compl-trans", "#3fbf0080");
    
    root_doc.style.setProperty("--pink", "#ff40c0");
    root_doc.style.setProperty("--pink-compl", "#00bf3f");
    root_doc.style.setProperty("--pink-trans", "#ff40c080");
    root_doc.style.setProperty("--pink-compl-trans", "#00bf3f80");
    
    root_doc.style.setProperty("--grassgreen", "#c0ff40");
    root_doc.style.setProperty("--grassgreen-compl", "#3f00bf");
    root_doc.style.setProperty("--grassgreen-trans", "#c0ff4080");
    root_doc.style.setProperty("--grassgreen-compl-trans", "#3f00bf80");
    
    root_doc.style.setProperty("--gold", "#ffc040");
    root_doc.style.setProperty("--gold-compl", "#003fbf");
    root_doc.style.setProperty("--gold-trans", "#ffc04080");
    root_doc.style.setProperty("--gold-compl-trans", "#003fbf80");
    
}

function to_light() {
    
    root_doc.style.setProperty("--black", "#ffffff");
    root_doc.style.setProperty("--black-compl", "#000000");
    root_doc.style.setProperty("--black-trans", "#ffffff80");
    root_doc.style.setProperty("--black-compl-trans", "#00000080");
    
    root_doc.style.setProperty("--dark", "#bfbfbf");
    root_doc.style.setProperty("--dark-compl", "#404040");
    root_doc.style.setProperty("--dark-trans", "#bfbfbf80");
    root_doc.style.setProperty("--dark-compl-trans", "#40404080");
    
    root_doc.style.setProperty("--gray", "#7f7f7f");
    root_doc.style.setProperty("--gray-compl", "#808080");
    root_doc.style.setProperty("--gray-trans", "#7f7f7f80");
    root_doc.style.setProperty("--gray-compl-trans", "#80808080");
    
    root_doc.style.setProperty("--lightgray", "#3f3f3f");
    root_doc.style.setProperty("--lightgray-compl", "#c0c0c0");
    root_doc.style.setProperty("--lightgray-trans", "#3f3f3f80");
    root_doc.style.setProperty("--lightgray-compl-trans", "#c0c0c080");
    
    root_doc.style.setProperty("--white", "#000000");
    root_doc.style.setProperty("--white-compl", "#ffffff");
    root_doc.style.setProperty("--white-trans", "#00000080");
    root_doc.style.setProperty("--white-compl-trans", "#ffffff80");
    
    root_doc.style.setProperty("--red", "#3fbfbf");
    root_doc.style.setProperty("--red-compl", "#c04040");
    root_doc.style.setProperty("--red-trans", "#3fbfbf80");
    root_doc.style.setProperty("--red-compl-trans", "#c0404080");
    
    root_doc.style.setProperty("--green", "#bf3fbf");
    root_doc.style.setProperty("--green-compl", "#40c040");
    root_doc.style.setProperty("--green-trans", "#bf3fbf80");
    root_doc.style.setProperty("--green-compl-trans", "#40c04080");
    
    root_doc.style.setProperty("--blue", "#bfbf3f");
    root_doc.style.setProperty("--blue-compl", "#4040c0");
    root_doc.style.setProperty("--blue-trans", "#bfbf3f80");
    root_doc.style.setProperty("--blue-compl-trans", "#4040c080");
    
    root_doc.style.setProperty("--aqua", "#bf3f00");
    root_doc.style.setProperty("--aqua-compl", "#40c0ff");
    root_doc.style.setProperty("--aqua-trans", "#bf3f0080");
    root_doc.style.setProperty("--aqua-compl-trans", "#40c0ff80");
    
    root_doc.style.setProperty("--emerald", "#bf003f");
    root_doc.style.setProperty("--emerald-compl", "#40ffc0");
    root_doc.style.setProperty("--emerald-trans", "#bf003f80");
    root_doc.style.setProperty("--emerald-compl-trans", "#40ffc080");
    
    root_doc.style.setProperty("--violet", "#3fbf00");
    root_doc.style.setProperty("--violet-compl", "#c040ff");
    root_doc.style.setProperty("--violet-trans", "#3fbf0080");
    root_doc.style.setProperty("--violet-compl-trans", "#c040ff80");
    
    root_doc.style.setProperty("--pink", "#00bf3f");
    root_doc.style.setProperty("--pink-compl", "#ff40c0");
    root_doc.style.setProperty("--pink-trans", "#00bf3f80");
    root_doc.style.setProperty("--pink-compl-trans", "#ff40c080");
    
    root_doc.style.setProperty("--grassgreen", "#3f00bf");
    root_doc.style.setProperty("--grassgreen-compl", "#c0ff40");
    root_doc.style.setProperty("--grassgreen-trans", "#3f00bf80");
    root_doc.style.setProperty("--grassgreen-compl-trans", "#c0ff4080");
    
    root_doc.style.setProperty("--gold", "#003fbf");
    root_doc.style.setProperty("--gold-compl", "#ffc040");
    root_doc.style.setProperty("--gold-trans", "#003fbf80");
    root_doc.style.setProperty("--gold-compl-trans", "#ffc04080");
    
}

var change_theme_button = document.getElementById("changethemebutton");
change_theme_button.addEventListener("click", change_theme);