pub static CREATE_TABLES: &str = r#"
CREATE TABLE audio_source(
    id              INTEGER PRIMARY KEY,
    file            TEXT,
    time_start      REAL,
    time_end        REAL
);

CREATE TABLE track(
    id              INTEGER PRIMARY KEY,
    title           TEXT COLLATE NOCASE,
    source          INTEGER REFERENCES audio_source,
    album           INTEGER REFERENCES album,
    number          TEXT COLLATE NOCASE,
    artist          INTEGER REFERENCES group
);

CREATE TABLE track_tag(
    track           INTEGER NOT NULL REFERENCES track,
    tag             INTEGER NOT NULL REFERENCES tag
);
CREATE INDEX track_tag.tracks ON track_tag(track);
CREATE INDEX track_tag.tag ON track_tag(tags);

CREATE TABLE track_role(
    track           INTEGER NOT NULL REFERENCES track,
    role            TEXT,
    artist          INTEGER NOT NULL REFERENCES group
);
CREATE INDEX track_role.tracks ON track_role(track);
CREATE INDEX track_role.artists ON track_role(artist);

CREATE TABLE album(
    id              INTEGER PRIMARY KEY,
    title           TEXT COLLATE NOCASE,
    artist          INTEGER REFERENCES group
);

CREATE TABLE album_tag(
    album           INTEGER NOT NULL REFERENCES album,
    tag             INTEGER NOT NULL REFERENCES tag
);
CREATE INDEX album_tag.albums ON album_tag(album);
CREATE INDEX album_tag.tag ON album_tag(tags);

CREATE TABLE group(
    id              INTEGER PRIMARY KEY,
    name            TEXT COLLATE NOCASE
);

CREATE TABLE group_tag(
    group           INTEGER NOT NULL REFERENCES group,
    tag             INTEGER NOT NULL REFERENCES tag
);
CREATE INDEX group_tag.groups ON group_tag(group);
CREATE INDEX group_tag.tag ON group_tag(tags);

CREATE TABLE group_member(
    group           INTEGER NOT NULL REFERENCES group,
    role            TEXT,
    member          INTEGER NOT NULL REFERENCES group
);
CREATE INDEX group_member.groups ON group_member(group);
CREATE INDEX group_member.members ON group_member(member);

CREATE TABLE tag_category(
    id              INTEGER PRIMARY KEY,
    name            TEXT COLLATE NOCASE
);

CREATE TABLE tag(
    id              INTEGER PRIMARY KEY,
    category        INTEGER NOT NULL REFERENCES tag_category,
    value           TEXT COLLATE NOCASE
);
CREATE INDEX tag.categories ON tag(category);
"#;
