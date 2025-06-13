import { A, useParams } from "@solidjs/router";
import { createEffect, createSignal, For, Show } from "solid-js";
import { Oval } from "solid-spinner";
import { DomainAnalytics, getDomainAnalytics } from "../db/analytics";
import { domains, DomainTableEntry } from "../db/resources/domain_data";

export function DomainView() {
    const params = useParams();
    const [data, setData] = createSignal<DomainTableEntry>();
    const [analytics, setAnalytics] = createSignal<DomainAnalytics>();
    const [loading, setLoading] = createSignal(true);

    createEffect(async () => {
        setLoading(true);
        if (domains.state === "ready") {
            setData(domains().find(d => d.domain === params.domain));
            setAnalytics(await getDomainAnalytics(params.domain));
        }
        setLoading(false);
    })

    return <div>
        <h1 class="flex items-center gap-4">
            {params.domain}
            <Show when={loading()}>
                <Oval width={25} height={25} />
            </Show>
        </h1>
        <br />
        <Show when={data() && !loading()} 
            fallback={loading() ?
                null :
                <p class="text-red-500">Invalid domain.</p>
            }
        >
            <p>Record created @ {new Date(data().created_at).toUTCString()}</p>
            <br />
            <p>Owned by <span class="code">{data().id}</span> <i>(You)</i></p>
            <br />
            <br />
            <Show when={analytics()}>
                <h3 class="font-semibold">Indexed pages</h3>
                <br />
                <div class="w-full flex flex-col">
                    <div class="grid grid-cols-3 bg-neutral-100 dark:bg-neutral-900 border">
                        <p class="font-semibold p-4">Title</p>
                        <p class="font-semibold p-4">URL</p>
                        <p class="font-semibold p-4">Indexed description</p>
                    </div>
                    <For each={analytics().indexed_pages}>
                        {page => <div class="grid grid-cols-3 border-t border-l border-b">
                            <A class="pl-2 flex items-center border-r text-nowrap overflow-hidden"
                                href={page.url}
                            >
                                {new URL(page.url).pathname}
                            </A>
                            <p class="p-2 flex items-center border-r">
                                {page.title}
                            </p>
                            <p class="p-2 flex items-center border-r">
                                {page.description}
                            </p>
                        </div>}
                    </For>
                </div>
            </Show>
        </Show>
    </div>
}
