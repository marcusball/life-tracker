document.addEventListener("DOMContentLoaded", function (event) {
    console.log("A'ight, DOM is loaded, where's the SUB? ;)");

    displayCounters();
});

function displayCounters() {
    Promise.all([loadCounters(), compileCounterTemplate()])
        .then((promises) => {
            let counters = promises[0];
            let template = promises[1];

            let html = template({ counters: counters });

            let counterContainer = document.getElementById('js-counters');
            counterContainer.insertAdjacentHTML('beforeend', html);
        });
}

/**
 * Loads an array of counters. 
 * @returns Promise a Promise of an array of Counter objects. 
 */
function loadCounters() {
    return fetch('/counters')
        .then((response) => { return response.json(); });
}

/**
 * Loads the template for displaying a list of counters. 
 * @returns Promise a Promise of a compiled Handlebars template. 
 */
function compileCounterTemplate() {
    return fetch('/static/templates/index-counters.html.hbs')
        .then((response) => { return response.text(); })
        .then(Handlebars.compile);
}