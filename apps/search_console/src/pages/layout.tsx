import { A, RouteSectionProps } from "@solidjs/router";

export function RootLayout(props: RouteSectionProps<any>) {
    return <div class="w-full min-h-dvh flex flex-col">
        <div class="w-full border-b border-gray-500 p-2 flex justify-between">
            <p class="logo_like">
                <a onClick={() => window.location.assign("/")} href="/" class="logo_like hover:text-red-300">JOOGLE</a>
                {" "} Search Console
            </p>
            <nav class="flex items-center gap-2">
                <A href="/search/console" activeClass="active" end>Dashboard</A>
                <A href="/search/console/jobs" activeClass="active" end>Jobs</A>
            </nav>
            <p class="logo_like text-transparent select-none">JOOGLE Search Console</p>
        </div>
        <div class="flex flex-1 justify-self-stretch">
            <div class="p-2 min-w-[250px] max-w-[350px] w-[20vw] max-lg:hidden border-r border-gray-500">
                OWNED DOMAINS
            </div>
            <div class="p-2 w-full">
            {props.children}
            </div>
        </div>
    </div>
}
