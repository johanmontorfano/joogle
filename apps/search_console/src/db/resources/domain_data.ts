import { createResource } from "solid-js";
import { supabase, userData } from "../supabase";

export interface DomainTableEntry {
    id: string;
    created_at: number;
    owned_by: string;
    domain: string;
}

/// Will load all domains owned by the user from supabase.
export async function fetchDomains(): Promise<DomainTableEntry[]> {
    const { data: { user } } = await supabase.auth.getUser();

    if (user === null)
        return [];

    const {data, error} = await supabase.from("domains")
        .select("*")
        .eq("owned_by", user.id);

    if (error)
        throw new Error(error.message);

    return data || [];
}

export const [domains, { refetch: getDomains }] = createResource(fetchDomains);
