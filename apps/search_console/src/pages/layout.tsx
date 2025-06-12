import { RouteSectionProps, useNavigate } from "@solidjs/router";
import { IoPersonOutline } from "solid-icons/io";
import { createSignal } from "solid-js";
import { DomainsList } from "../components/domains_list";
import { Popup } from "../components/popup";
import { supabase, userData } from "../db/supabase";

export function RootLayout(props: RouteSectionProps<any>) {
    const [showProfile, setShowProfile] = createSignal(false);
    const navigate = useNavigate();

    return <div class="w-full min-h-dvh flex flex-col">
        <div class="w-full border-b border-gray-500 p-2 flex justify-between items-center">
            <p class="logo_like">
                <a onClick={() => window.location.assign("/")} href="/" class="logo_like hover:text-red-300">JOOGLE</a>
                {" "} Search Console
            </p>
            <IoPersonOutline size={20}
                class="cursor-pointer"
                onClick={() => setShowProfile(true)}
            />
        </div>
        <div class="flex flex-1 justify-self-stretch">
            <div class="p-2 min-w-[250px] max-w-[350px] w-[20vw] max-lg:hidden border-r border-gray-500">
                <DomainsList />
            </div>
            <div class="p-2 w-full dark:bg-neutral-950">
                {props.children}
            </div>
        </div>
        <Popup show={showProfile()} onClose={() => setShowProfile(false)}>
            <div>
                <p>Account: {userData().email}</p>
            </div>
            <div class="w-full flex justify-end">
                <button class="input submit hover:bg-red-500"
                    onClick={async () => {
                        await supabase.auth.signOut();
                        navigate("/search/console/auth");
                    }}
                >
                    Log out
                </button>
            </div>
        </Popup>
    </div>
}
