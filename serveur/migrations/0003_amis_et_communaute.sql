create table amis (
    demandeur_id uuid not null references comptes (id) on delete cascade,
    receveur_id uuid not null references comptes (id) on delete cascade,
    statut text not null default 'attente',
    cree_le timestamptz not null default now(),
    primary key (demandeur_id, receveur_id)
);
create index amis_receveur on amis (receveur_id);

create table partages_sets (
    set_id uuid not null references sets (id) on delete cascade,
    destinataire_id uuid not null references comptes (id) on delete cascade,
    cree_le timestamptz not null default now(),
    primary key (set_id, destinataire_id)
);
create index partages_destinataire on partages_sets (destinataire_id);

alter table sets add column visibilite text not null default 'prive';
alter table sets add column description text;

create table favoris_sets (
    compte_id uuid not null references comptes (id) on delete cascade,
    set_id uuid not null references sets (id) on delete cascade,
    cree_le timestamptz not null default now(),
    primary key (compte_id, set_id)
);
create index favoris_compte on favoris_sets (compte_id);
