import { useParams } from "@solidjs/router";
import { createEffect, createSignal, Show } from "solid-js";
import { domains, DomainTableEntry } from "../db/resources/domain_data";

export function DomainView() {
    const { domain } = useParams();
    const [data, setData] = createSignal<DomainTableEntry>();

    createEffect(() => {
        if (domains.state === "ready")
            setData(domains().find(d => d.domain === domain));
    })

    return <div>
        <h1>{domain}</h1>
        <br />
        <Show when={data()} fallback={<p class="text-red-500">Invalid domain.</p>}>
            <p>Record created @ {new Date(data().created_at).toUTCString()}</p>
            <br />
            <p>Owned by <span class="code">{data().id}</span> <i>(You)</i></p>
            <br />
            <br />
            <p>Domain analytics and controls are coming soon !</p>
        </Show>
    </div>
}
