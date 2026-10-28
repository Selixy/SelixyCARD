create table images (
    sha256 text primary key,
    octets bytea not null,
    auteur_id uuid not null references comptes (id) on delete cascade,
    cree_le timestamptz not null default now()
);

create table sets (
    id uuid primary key,
    auteur_id uuid not null references comptes (id) on delete cascade,
    nom text not null,
    dos text,
    cree_le timestamptz not null default now(),
    modifie_le timestamptz not null default now()
);
create index sets_auteur on sets (auteur_id);

create table cartes_set (
    set_id uuid not null references sets (id) on delete cascade,
    id text not null,
    rang integer not null,
    genre text not null,
    couleur text,
    valeur integer,
    image text,
    primary key (set_id, id)
);
