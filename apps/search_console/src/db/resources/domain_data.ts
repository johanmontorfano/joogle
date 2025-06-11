import { createResource } from "solid-js";
import { supabase, userData } from "../supabase";

export interface DomainTableEntry {
    id: string;
    created_at: number;
    owned_by: string;
    domain: string;
}

export async function fetchDomains(): Promise<DomainTableEntry[]> {
    const {data, error} = await supabase.from("domains")
        .select("*")
        .eq("owned_by", userData().id);

    if (error)
        throw new Error(error.message);

    return data;
}

export const [domains, { refetch: getDomains }] = createResource(fetchDomains);
