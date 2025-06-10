import { createSignal } from "solid-js";

// Will provide with indexing system status. It will also process a tendency
// that highlights the adding or removal rate of URLs from the queue. If the
// tendency goes up, it means the server may overload at some point, else it
// means there is way less traffic, and it may be a great idea to index sthg
export function useIndexSysStats() {
    const [queueLength, setQueueLength] = createSignal(0);
    const [indexedUrls, setIndexedUrls] = createSignal(0);
    // WARN: The tendency is expressed in links/s
    const [tendency, setTendency] = createSignal(0);

    // NOTE: A point is pushed every 5 seconds
    const dataPoints = [];

    async function getStatus() {
        const status = await (await fetch("/api/index_sys_status")).json();

        setQueueLength(status.queue_length);
        setIndexedUrls(status.indexed_urls);
        dataPoints.push(status.queue_length);
        
        const total = dataPoints
            .map((pt, i) => i > 0 ? pt - dataPoints[i - 1] : 0)
            .reduce((prev, curr) => prev + curr);
        setTendency(total / (dataPoints.length * 5 - 1) / 5);
    }

    getStatus();

    setInterval(getStatus, 5000);

    return [queueLength, indexedUrls, tendency];
}
