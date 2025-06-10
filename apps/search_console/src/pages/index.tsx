import { useIndexSysStats } from "../hooks/use_stats"

export function RootIndex() {
    const [queueLength, indexedUrls, tendency] = useIndexSysStats();

    return <div>
        <div>
            <p>Queue length: {queueLength()} ({tendency()}/s)</p>
            <p>Indexed URLs: {indexedUrls()}</p>
        </div>
    </div>
}
