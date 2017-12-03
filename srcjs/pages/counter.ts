class Counter {
    id: number;
    url: string;
    name: string;
    unit: string;
    events: string;

    getEvents(): Promise<CounterEvent[]> {
        return fetch(this.events, {
            headers: new Headers({
                'Accept': 'application/json'
            })
        })
            .then((response) => response.json())
            .then((json: object[]) => {
                return json.map((obj) => Object.assign(new CounterEvent(), obj))
            })
            .then((events: CounterEvent[]) => {
                console.log(events);
                return events;
            });
    }
}

class CounterEvent {
    id: number;
    quantity: number;
    timestamp: string;
}

document.addEventListener('DOMContentLoaded', (event) => {
    let url = window.location.pathname;

    fetch(url, {
        headers: new Headers({
            'Accept': 'application/json'
        })
    })
        .then((response) => response.json())
        .then((json: object) => Object.assign(new Counter(), json))
        .then((counter: Counter) => counter.getEvents())
        .then((events: CounterEvent[]) => console.log(events));
})
