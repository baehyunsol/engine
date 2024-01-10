"use strict";

var mdxt_sidebar_open = false;

function mdxt_toggle_sidebar() {

  if (mdxt_sidebar_open) {
    mdxt_close_sidebar();
  }

  else {
    document.getElementById("mdxt-sidebar").style.width = "var(--sidebar-width)";
    document.getElementById("mdxt-sidebar-toggle").style.left = "var(--sidebar-width)";
    document.getElementById("mdxt-sidebar-button-content").innerHTML = "≪";
    mdxt_sidebar_open = true;
  }

}

function mdxt_close_sidebar() {
  document.getElementById("mdxt-sidebar").style.width = "0";
  document.getElementById("mdxt-sidebar-toggle").style.left = "0";
  document.getElementById("mdxt-sidebar-button-content").innerHTML = "≫";
  mdxt_sidebar_open = false;
}
