-- Add migration script here
CREATE TABLE IF NOT EXISTS buyer (
  id uuid primary key,
  contact_email text not null,
  created timestamp without time zone not null,
  updated timestamp without time zone not null
);

CREATE TABLE IF NOT EXISTS site (
  id uuid primary key,
  contact_email text not null,
  url text not null,
  created timestamp without time zone not null,
  updated timestamp without time zone not null
);

CREATE TABLE IF NOT EXISTS ad (
  id uuid primary key,
  active boolean not null,
  asset_url_sm text not null,
  asset_url_lg text not null,
  clickthrough_url text not null,
  created timestamp without time zone not null,
  updated timestamp without time zone not null,
  buyer_id uuid not null references buyer(id) on delete cascade
);

CREATE TABLE IF NOT EXISTS event (
  id uuid primary key,
  event varchar(10) not null,
  occurred timestamp without time zone not null,
  source_ip inet not null,
  metadata jsonb not null,
  ad_id uuid not null references ad(id),
  site_id uuid not null references site(id)
);
