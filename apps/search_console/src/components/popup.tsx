import { IoCloseOutline } from "solid-icons/io";
import { JSX, Show } from "solid-js";
import { Portal } from "solid-js/web";

export function Popup(props: {
    children: JSX.Element,
    show: boolean,
    onClose: () => void
}) {
    return <Portal>
        <Show when={props.show}>
            <div class="flex items-center justify-center w-full h-dvh absolute top-0 left-0 backdrop-blur-md">
                <div class="border shadow-md border-neutral-600 bg-neutral-800 rounded-lg w-full max-w-[400px]">
                    <div class="flex justify-end w-full p-2 border-b border-gray-500">
                        <IoCloseOutline onClick={props.onClose} 
                            size={24}
                            class="cursor-pointer"
                        />
                    </div>
                    <div class="p-2">
                        {props.children}
                    </div>
                </div>
            </div>
        </Show>
    </Portal>
}
