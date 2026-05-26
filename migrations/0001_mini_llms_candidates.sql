create schema if not exists mini_llms;

create table if not exists mini_llms.runtime_observations (
  id uuid primary key,
  run_id text not null,
  runtime text not null,
  model text,
  provider_shape text not null,
  extraction_status text not null,
  raw_response jsonb,
  created_at timestamptz not null default now()
);

create table if not exists mini_llms.logline_candidates (
  id uuid primary key,
  observation_id uuid references mini_llms.runtime_observations(id),
  tuple_hash text not null,
  content_hash text not null unique,
  canon_version text not null,
  schema_version text not null,
  who text not null,
  did text not null,
  this jsonb not null,
  "when" text not null,
  confirmed_by jsonb not null,
  if_ok text not null,
  if_doubt text not null,
  if_not text not null,
  status text not null,
  created_at timestamptz not null default now()
);
create index if not exists idx_logline_tuple_hash on mini_llms.logline_candidates(tuple_hash);

create table if not exists mini_llms.findings (id uuid primary key, candidate_id uuid, payload jsonb not null, created_at timestamptz not null default now());
create table if not exists mini_llms.ghosts (id uuid primary key, candidate_id uuid, reason text not null, created_at timestamptz not null default now());
create table if not exists mini_llms.profile_evaluations (id uuid primary key, candidate_id uuid, metric_payload jsonb not null, decision text not null, created_at timestamptz not null default now());
