function collapse_table(n) {
    var head = document.getElementById("table-collapse-toggle-" + n);
    head.classList.toggle("collapsed");

    var content = document.getElementById("collapsible-table-" + n);
    content.classList.toggle("invisible");
}