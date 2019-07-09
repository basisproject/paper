--------------------------------------------------------------------------------
-- v2
--------------------------------------------------------------------------------

-- bank
create type account_type as enum ('company', 'person', 'region');
create table if not exists bank_accounts (id uuid primary key, entity_type account_type not null, entity_id uuid not null, funds numeric(20, 2) not null default 0, public_key text not null, created timestamp with time zone not null default current_timestamp);
create unique index if not exists bank_entity_type_entity_id on bank_accounts (entity_type, entity_id);

-- companies
create table if not exists companies (id uuid primary key, worker_owned bool, name varchar(256) not null, description text, created timestamp with time zone not null default current_timestamp);

create table if not exists companies_people_link (id bigserial primary key, companies_id uuid not null, people_id uuid not null, company_position_id bigint not null, created timestamp with time zone not null default current_timestamp);
create unique index if not exists companies_people_link on companies_people_link (people_id, company_position_id);

create table if not exists companies_products (id uuid primary key, companies_id uuid not null, inventory bigint not null default 0, name varchar(256) not null, description text, created timestamp with time zone not null default current_timestamp);
create index if not exists companies_products_companies_id on companies_products (companies_id);

-- config
create table if not exists config (id bigserial primary key, key varchar(256), value jsonb, created timestamp with time zone not null default current_timestamp);
create unique index if not exists config_key on config (key);

-- housing
create table if not exists housing (id uuid primary key, companies_id uuid not null, housing_type varchar(64) not null, address varchar(256), address2 varchar(256), city varchar(256), state varchar(256), created timestamp with time zone not null default current_timestamp);
create index if not exists housing_housing_type on housing (housing_type);
create index if not exists housing_companies_id on housing (companies_id);

create table if not exists housing_units (id bigserial primary key, housing_id uuid not null, size double precision not null, created timestamp with time zone not null default current_timestamp);
create index if not exists housing_units_housing_id on housing_units (housing_id, size);

create table if not exists housing_people_link (id bigserial primary key, housing_units_id bigint not null, people_id uuid not null, created timestamp with time zone not null default current_timestamp);
create unique index if not exists housing_people_link on housing_people_link (housing_units_id, people_id);

-- people
create table if not exists people (id uuid primary key, fullname varchar(256) not null, dob timestamp with time zone not null, created timestamp with time zone not null default current_timestamp);
create index if not exists people_dob on people (dob);

--------------------------------------------------------------------------------
-- v1
--------------------------------------------------------------------------------

create table if not exists companies (id bigserial primary key, name varchar(256) not null, created timestamp with time zone not null default current_timestamp);
create table if not exists companies_outputs (id bigserial primary key, global_export_id bigint not null, amount_per_day double precision not null, created timestamp with time zone not null default current_timestamp);
create table if not exists companies_inputs (id bigserial primary key, global_import_id bigint not null, amount_per_day double precision not null, created timestamp with time zone not null default current_timestamp);
create table if not exists companies_people_link (id bigserial primary key, company_id bigint not null, people_id bigint not null, company_position_id bigint not null, created timestamp with time zone not null default current_timestamp);
create table if not exists companies_positions (id bigserial primary key, name varchar(256) not null, description text not null, created timestamp with time zone not null default current_timestamp);
create table if not exists companies_positions_abilities_link (id bigserial primary key, name varchar(256) not null, companies_positions_id bigint not null, people_abilities_id bigint not null, created timestamp with time zone not null default current_timestamp);
create index if not exists companies_outputs_global_id on companies_outputs (global_export_id);
create index if not exists companies_inputs_global_id on companies_inputs (global_import_id);
create unique index if not exists companies_people_link on companies_people_link (people_id, company_position_id);
create unique index if not exists companies_positions_abilities_link on companies_positions_abilities_link (companies_positions_id, people_abilities_id);

create table if not exists config (id bigserial primary key, key varchar(256), value jsonb, created timestamp with time zone not null default current_timestamp);
create unique index if not exists config_key on config (key);

create table if not exists events (id bigserial primary key, users_id bigint, users_tokens_id bigint, action varchar(64), item_type varchar(64), item_id bigint, created timestamp with time zone not null default current_timestamp);
create index if not exists events_users_id on events(users_id, users_tokens_id);
create index if not exists events_item_audit on events(item_type, item_id);

create table if not exists housing (id bigserial primary key, housing_types_id bigint not null, houses bigint not null, name varchar(256) not null, description text, address varchar(256), address2 varchar(256), city varchar(256), state varchar(256), created timestamp with time zone not null default current_timestamp);
create table if not exists housing_people_link (id bigserial primary key, housing_id bigint not null, people_id bigint not null, created timestamp with time zone not null default current_timestamp);
create table if not exists housing_types (id bigserial primary key, name varchar(256) not null, description text, created timestamp with time zone not null default current_timestamp);
create index if not exists housing_types on housing (housing_types_id);
create unique index if not exists housing_people_link on housing_people_link (housing_id, people_id);

create table if not exists people (id bigserial primary key, global_id varchar(256), fullname varchar(256) not null, dob timestamp with time zone not null, created timestamp with time zone not null default current_timestamp);
create table if not exists people_abilities (id bigserial primary key, global_people_abilities_id bigint, name varchar(256) not null, description text, created timestamp with time zone not null default current_timestamp);
create table if not exists people_abilities_link (id bigserial primary key, people_id bigint not null, people_abilities_id bigint not null, created timestamp with time zone not null default current_timestamp);
create unique index if not exists people_global_id on people (global_id);
create index if not exists people_dob on people (dob);
create index if not exists people_abilities_global_id on people_abilities (global_people_abilities_id);
create unique index if not exists people_abilities_link on people_abilities_link (people_id, people_abilities_id);

create table if not exists users (id bigserial primary key, username varchar(256) not null, password text not null, active bool default false, fullname varchar(256), settings jsonb, created timestamp with time zone not null default current_timestamp);
create table if not exists users_permissions (id bigserial primary key, name varchar(256) not null, description text not null, created timestamp with time zone not null default current_timestamp);
create table if not exists users_permissions_roles_link (id bigserial primary key, users_permissions_id bigint not null, users_roles_id bigint not null, created timestamp with time zone not null default current_timestamp);
create table if not exists users_roles (id bigserial primary key, name varchar(256) not null, description text not null, created timestamp with time zone not null default current_timestamp);
create table if not exists users_tokens (id bigserial primary key, user_id bigint not null, users_roles_id bigint not null, active bool default false, created timestamp with time zone not null default current_timestamp);
create unique index if not exists users_username on users (username);
create unique index if not exists users_permissions_roles_link on users_permissions_roles_link (users_permissions_id, users_roles_id);

