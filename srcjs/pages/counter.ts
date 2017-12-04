import * as UrlPattern from 'url-pattern';

/**
 * Represents a single Counter of events. 
 */
class Counter {
    /**
     * URL pattern for accessing a Counter
     */
    public static readonly urlPattern = new UrlPattern('/counter/:url');

    /**
     * Query the API to get information about a specific counter. 
     * @param url The url for the desired counter
     */
    public static getFromUrl(url: string): Promise<Counter> {
        return fetch(url, {
            headers: new Headers({
                Accept: 'application/json',
            }),
        })
            .then((response) => response.json())
            .then((json: object) => Object.assign(new Counter(), json));
    }

    /** The ID of this counter */
    public id: number;

    /** The string used to identify this counter in urls */
    public url: string;

    /** The human-readable name of this counter */
    public name: string;

    /** The unit of measurement in this counter */
    public unit: string;

    /** URL to query for the events within this counter */
    public events: string;

    /**
     * Query the API for this counter's events.
     * @returns Promise<CounterEvent[]> An array of events
     */
    public getEvents(): Promise<CounterEvent[]> {
        return fetch(this.events, {
            headers: new Headers({
                Accept: 'application/json',
            }),
        })
            .then((response) => response.json())
            .then((json: object[]) => {
                // Convert the `object` to a `CounterEvent` object.
                return json.map((obj) => Object.assign(new CounterEvent(), obj));
            });
    }
}

/**
 * A single event within a Counter
 */
class CounterEvent {
    /** The unique ID for this event */
    public id: number;

    /** The quantity measured during this event. */
    public quantity: number;

    /** The timestamp of when this event occurred. */
    public timestamp: string;
}

document.addEventListener('DOMContentLoaded', (event) => {
    const url = window.location.pathname;

    Counter.getFromUrl(url)
        .then((counter: Counter) => counter.getEvents())
        .then((events: CounterEvent[]) => {
            console.log(events);
        });
});
