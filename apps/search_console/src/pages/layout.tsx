import { RouteSectionProps } from "@solidjs/router";

export function RootLayout(props: RouteSectionProps<any>) {
    return <div class="w-full min-h-dvh">
        <div class="w-full border-b border-gray-500 p-2">
            <p class="logo_like">JOOGLE Search Console</p>
        </div>
        <div>
            {props.children}
        </div>
    </div>
}
