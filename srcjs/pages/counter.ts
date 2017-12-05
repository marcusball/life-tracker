import * as UrlPattern from 'url-pattern';

// TODO: See if this can be changed to not need a relative path
import * as template from '../../static/templates/counter-events.html.hbs';

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

/**
 * Load a Counter and its events, and then displays them on the page. 
 * @param counterUrl The API url form querying a counter; ex: '/counter/my-counter'
 */
function loadEvents(counterUrl: string): Promise<any> {
    return Counter.getFromUrl(counterUrl)
        .then(displayEvents);
}

/**
 * Load events from a counter, then display them in a table. 
 * @param counter The Counter which should have its events displayed. 
 */
function displayEvents(counter: Counter): Promise<any> {
    return counter.getEvents()
        .then((events: CounterEvent[]) => {
            const html = template({ counter, events });

            const eventsContainer = document.getElementById('events');
            if (!eventsContainer) { return; }

            // Murder all of this element's innocent children
            eventsContainer.innerHTML = '';

            // Fill the container with some brand new content
            eventsContainer.insertAdjacentHTML('beforeend', html);
        });
}

function onNewEventSubmit(event: Event) {
    event.preventDefault();

    const formData = new FormData(event.target as HTMLFormElement);

    // Because Rocket currently does not support `multipart/form-data`
    const urlData = formDataToUrlParams(formData);

    fetch(window.location.pathname + '/events', {
        method: 'POST',
        credentials: 'same-origin',
        body: urlData,
    })
        .then((res) => res.json())
        .then((json) => { console.log(json); })
        // Refresh the events list
        .then(() => loadEvents(window.location.pathname));
}

/**
 * Convert a FormData instance into a URLSearchParams object by copying over keys and values. 
 * @param formData 
 */
function formDataToUrlParams(formData: FormData): URLSearchParams {
    // Reduce the form's key-value pairs into a URLSearchParams object
    return [...formData.entries()] // spread the form entries iterator into an Array
        .reduce((urlParams, pair) => {
            // Skip 'File' types, for now, at least. 
            if (!(pair[1] instanceof File)) {
                urlParams.append(pair[0], pair[1] as string);
            }
            return urlParams;
        }, new URLSearchParams());
}

document.addEventListener('DOMContentLoaded', (event) => {
    const url = window.location.pathname;

    loadEvents(url);

    const formId: any = 'new_event_form';
    (document.forms[formId] as HTMLFormElement)
        .addEventListener('submit', onNewEventSubmit);
});
