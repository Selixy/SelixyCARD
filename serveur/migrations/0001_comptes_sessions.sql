create table comptes (
    id uuid primary key,
    pseudo text not null,
    mot_de_passe text not null,
    cree_le timestamptz not null default now()
);
create unique index comptes_pseudo_unique on comptes (lower(pseudo));

create table sessions (
    empreinte bytea primary key,
    compte_id uuid not null references comptes (id) on delete cascade,
    expire_le timestamptz not null
);
create index sessions_compte on sessions (compte_id);
