import { A } from "@solidjs/router";
import { For } from "solid-js";
import { domains } from "../db/resources/domain_data";

export function DomainsList() {
    return <div>
        <For each={domains()}>
            {d => <p>{d.domain}</p>}
        </For> 
        <A href="/search/console/new">New Domain</A>
    </div>
}
