class Counter {
    public id: number;
    public url: string;
    public name: string;
    public unit: string;
    public events: string;

    public getEvents(): Promise<CounterEvent[]> {
        return fetch(this.events, {
            headers: new Headers({
                Accept: 'application/json',
            }),
        })
            .then((response) => response.json())
            .then((json: object[]) => {
                // Convert the `object` to a `CounterEvent` object. 
                return json.map((obj) => Object.assign(new CounterEvent(), obj))
            });
    }
}

class CounterEvent {
    public id: number;
    public quantity: number;
    public timestamp: string;
}

document.addEventListener('DOMContentLoaded', (event) => {
    const url = window.location.pathname;

    fetch(url, {
        headers: new Headers({
            Accept: 'application/json',
        }),
    })
        .then((response) => response.json())
        .then((json: object) => Object.assign(new Counter(), json))
        .then((counter: Counter) => counter.getEvents())
        .then((events: CounterEvent[]) => console.log(events));
});
