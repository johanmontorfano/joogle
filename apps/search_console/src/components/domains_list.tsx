import { A } from "@solidjs/router";
import { IoAddOutline } from "solid-icons/io";
import { For } from "solid-js";
import { domains } from "../db/resources/domain_data";

export function DomainsList() {
    return <div>
        <A href="/search/console/new"
            class="w-full p-2 flex justify-between items-center rounded-lg border dark:border-neutral-700 hover:bg-neutral-100 hover:dark:bg-neutral-800 input"
        >
            New Domain
            <IoAddOutline size={22} />
        </A>
        <br />
        <p class="font-semibold">
            Owned domains
        </p>
        <div class="flex flex-col">
            <For each={domains()} fallback={<p>No domain owned :(</p>}>
                {d => <A href={`/search/console/${d.domain}`}>
                    {d.domain}
                </A>}
            </For> 
        </div>
    </div>
}
