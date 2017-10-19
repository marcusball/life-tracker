document.addEventListener("DOMContentLoaded", function (event) {
    console.log("A'ight, DOM is loaded, where's the SUB? ;)");

    loadCounters();
});

function loadCounters() {
    fetch('/counters')
        .then((response) => { return response.json(); })
        .then((counters) => { console.log(counters); });
}