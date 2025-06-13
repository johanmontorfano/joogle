import { SupabaseClient, User } from "@supabase/supabase-js";
import { createSignal } from "solid-js";

const url = import.meta.env.VITE_SUPABASE_URL;
const key = import.meta.env.VITE_SUPABASE_KEY;

export const [userData, setUserData] = createSignal<User>();

export const supabase = new SupabaseClient(url, key);
