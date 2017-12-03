interface Counter {
    id: number,
    url: string,
    name: string,
    unit: string,
    events: string
}

document.addEventListener('DOMContentLoaded', (event) => {
    let url = window.location.pathname;

    fetch(url, {
        headers: new Headers({
            'Accept': 'application/json'
        })
    })
        .then((response) => response.json())
        .then((counter: Counter) => { console.log(counter); });
})
