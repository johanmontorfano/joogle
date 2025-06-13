// Retrieves the analytics of a domain using a JWT and a domain name.

import { decodeJwt, SignJWT, UnsecuredJWT } from "jose";
import { API_ENDPOINT } from "..";
import { supabase } from "./supabase";

export interface DomainAnalytics {
    domain: string,
    owned_by: string,
    created_at: string,
    indexed_pages: {
        url: string,
        domain: string,
        title: string,
        description: string,
        ttr: string,
        loc: string
    }[]
}

// This route needs the user to be logged in.
export async function getDomainAnalytics(
    domain: string
): Promise<DomainAnalytics | null> {
    const user = await supabase.auth.getUser();
    const session = await supabase.auth.getSession();

    if (user.data === null || session.data === null) 
        return null;

    const session_data = decodeJwt(session.data.session.access_token);

    const email = user.data.user.email;
    const user_id = user.data.user.id;
    const session_id = session_data.session_id;

    const joogle_jwt = await new SignJWT({ email, user_id, session_id })
        .setProtectedHeader({ alg: "HS256" })
        .setIssuedAt()
        .setExpirationTime("1m")
        .sign(new TextEncoder().encode(import.meta.env.VITE_JWT_SECRET));

    const url = new URL(`${API_ENDPOINT}api/domain/get_analytics`);

    url.searchParams.set("domain", domain);

    const res = await fetch(url.toString(), {
        headers: {
            Authorization: joogle_jwt.toString()
        }
    });

    if (res.ok)
        return await res.json();
    return null;
}
