import { IoSendOutline } from "solid-icons/io";
import { createSignal, Show } from "solid-js"
import { API_ENDPOINT } from "..";
import { getDomains } from "../db/resources/domain_data";
import { supabase } from "../db/supabase";
import { Oval } from "solid-spinner";

export function NewDomain() {
    const [verificationKey, setVerificationKey] = createSignal("");
    const [domainVerified, setVerified] = createSignal<null | boolean>(null);
    const [loading, setLoading] = createSignal(false);
    const [domain, setDomain] = createSignal("");
    const [queuePosition, setQueuePosition] = createSignal(0);
    const [error, setError] = createSignal("");

    let domainInputSubmitRef: HTMLInputElement | null = null;

    // At this point, we expect the user to be logged in. Therefore, for
    // now, there will be no check for authentication. (valid for all submit
    // functions)
    // TODO: Consider checking if the User object is not null.

    async function retrieveVerificationKey(ev: SubmitEvent) {
        ev.preventDefault();
        setLoading(true);
        setDomain(ev.target[0].value);
    
        const user = await supabase.auth.getUser();
        const url = new URL(`${API_ENDPOINT}api/domain/get_ownership`);

        url.searchParams.set("domain", domain());
        url.searchParams.set("uid", user.data.user.id);

        const res = await fetch(url.toString());
        
        if (res.ok) {
            const json = await res.json();

            setDomain(json.for_domain);
            setVerificationKey(json.txt_record_content);
        }
        setLoading(false);
    }

    async function getDNSVerificationStatus() {
        setLoading(true);
    
        const user = await supabase.auth.getUser();
        const url = new URL(`${API_ENDPOINT}api/domain/check_dns_record`);

        url.searchParams.set("domain", domain());
        url.searchParams.set("uid", user.data.user.id);

        const res = await fetch(url.toString());
        
        if (res.ok) {
            const json = await res.json();

            setVerified(json.ownership_verified);
            setQueuePosition(json.queue_position),
            getDomains();
            setError("");
        }
        if (!domainVerified())
            setError("Cannot verify domain ownership.");
        setLoading(false);
    }

    return <div class="w-full flex flex-col items-center ">
        <Show when={error().length > 0}>
            <div class="w-full max-w-[1200px]">
                <p class="text-red-500">{error()}</p>
            </div>
        </Show>
        <div class="w-full max-w-[1200px]">
            <p class="step">
                <span class="step-count">1</span>
                Add Domain
            </p>
            <p class="mt-4 text-lg text-justify">
                Indexing your domain means a lot of things. The first and most
                obvious one is your domain becoming a few keywords away from
                anyone. But it also means search and indexing analytics
                will be enabled for your domain, letting you know everything
                from the number of search apparitions, to the number of found
                and indexed pages.
            </p>
            <p class="mt-4 text-lg text-justify">
                Therefore, we need to make sure your domain is linked to you
                and no one else through a secure procedure that requires you
                to edit your domain's DNS record.
            </p>
            <div class="callout">
                <p class="font-semibold">Useful links</p>
                <a href="https://www.cloudflare.com/learning/dns/dns-records/"
                    class="text-sm underline"
                >
                    What is a DNS record?
                </a>
                <br />
                <a href="https://www.aillum.com/blog/what-is-page-indexing/"
                    class="text-sm underline"
                >
                    What is page indexing?
                </a>
            </div>
            <p class="mt-4 font-semibold">Provide your domain name to begin</p>
            <form onSubmit={retrieveVerificationKey}>
                <div class="flex items-center">
                    <input type="text"
                        name="domain"
                        class="mr-2 w-[60%] w-min-[300px]"
                        value={domain()}
                        disabled={loading() || verificationKey().length > 0}
                    />
                    <input type="submit"
                        hidden
                        ref={r => domainInputSubmitRef = r}
                        disabled={loading() || verificationKey().length > 0}
                    />
                    <button class="input submit"
                        onClick={() => domainInputSubmitRef.click()}
                        disabled={loading() || verificationKey().length > 0}
                    >
                        {loading() ? <Oval width={24} height={24} /> :
                        <IoSendOutline size={24} />}
                    </button>
                </div>
            </form>
        </div>
        <Show when={verificationKey().length > 0}>
            <div class="w-full max-w-[1200px] mt-8">
                <p class="step">
                    <span class="step-count">2</span>
                    Confirm Ownership
                </p>
                <p class="mt-4 text-lg">
                    To confirm domain ownership add a <strong>TXT </strong> 
                    record with <strong>{verificationKey()} </strong> as 
                    content to the top-level DNS record of your domain. 
                </p>
                <p class="font-semibold mt-2">TXT record to add</p>
                <p class="code">
                    {domain()} 3600 IN TEXT "{verificationKey()}"
                </p>
                <div class="w-full flex justify-end">
                    <button disabled={loading() || domainVerified()}
                        onClick={getDNSVerificationStatus}
                        class="input submit min-w-[200px] flex justify-center" 
                    >
                        {loading() ? <Oval width={24} height={24} /> : "Verify"}
                    </button>
                </div>
            </div>
        </Show>
        <Show when={domainVerified()}>
            <div class="w-full max-w-[1200px] mt-8">
                <div>
                    <p class="step">
                        <span class="step-count">3</span>
                        Indexing
                    </p>
                </div>
                <p class="text-lg">Your domain is being indexed !</p>
                <p class="text-lg">
                    There are {queuePosition()} websites before yours :)
                </p>
            </div>
        </Show>
    </div>
}
