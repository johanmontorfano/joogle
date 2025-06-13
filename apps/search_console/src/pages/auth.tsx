import { useNavigate } from "@solidjs/router";
import { createSignal, Show } from "solid-js"
import { setUserData, supabase } from "../db/supabase";

export function Auth() {
    const [email, setEmail] = createSignal("");
    const [password, setPassword] = createSignal("");
    const [loading, setLoading] = createSignal(false);
    const [error, setError] = createSignal("");
    const navigate = useNavigate();

    async function onSubmit(ev: SubmitEvent) {
        ev.preventDefault();
        setLoading(true);

        const {data, error} = await supabase.auth.signInWithPassword({
            email: email(), password: password()
        });

        if (data.user !== null) {
            setUserData(data.user);
            navigate("/search/console");
        } else {
            setError(error.message);
            setLoading(false);
            setPassword("");
        }
    }

    return <div class="w-full h-dvh flex justify-center items-center">
        <div class="w-full max-w-[450px] border border-gray-500 rounded-lg">
            <div class="w-full border-b border-gray-500 p-2">
                <h1>Authenticate</h1>
                <p>Log in to continue with JOOGLE services</p>
                <Show when={error()}>
                    <p class="text-red-500">{error()}</p>
                </Show>
            </div>
            <form onSubmit={onSubmit}>
                <div class="w-full flex flex-col gap-2 rounded-lg p-2">
                    <input placeholder="Email" 
                        type="email"
                        name="email"
                        value={email()} 
                        onChange={e => setEmail(e.target.value)} 
                        disabled={loading()}
                        required
                    />
                    <input placeholder="Password"
                        type="password"
                        name="password" 
                        value={password()}
                        onChange={e => setPassword(e.target.value)}
                        disabled={loading()}
                        required
                    />
                </div>
                <div class="border-t border-gray-500 p-2 grid grid-cols-2 w-full gap-2">
                    <button class="input submit hover:bg-black"
                        onClick={() => location.assign("/")}
                        disabled={loading()}
                    >
                        Cancel
                    </button>
                    <input type="submit" 
                        value="Continue" 
                        class="hover:bg-blue-500" 
                        disabled={email().length < 4 || password().length < 1 || loading()}
                    />
                </div>
            </form>
        </div>
    </div>
}
