-- Supabase ready table with RLS
create table public.domains (
  id uuid not null default gen_random_uuid(),
  created_at timestamp with time zone not null default now(),
  owned_by uuid not null,
  domain text not null,
  constraint domains_pkey primary key (id)
) TABLESPACE pg_default;
