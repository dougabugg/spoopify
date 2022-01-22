use std::marker::PhantomData;

pub mod schema;

pub type Key = u64;

pub struct ForiegnKey<T> {
    _pd: PhantomData<Box<T>>,
    id: Option<Key>,
}

pub struct AudioSource {
    id: Key,
    file: String,
    time_start: f64,
    time_end: f64,
}

pub struct Track {
    id: Key,
    title: String,
    source: ForiegnKey<AudioSource>,
    album: ForiegnKey<Album>,
    number: String,
    artist: ForiegnKey<Group>,
}

pub struct TrackTag {
    track: ForiegnKey<Track>,
    tag: ForiegnKey<Tag>,
}

pub struct TrackRole {
    track: ForiegnKey<Track>,
    role: String,
    artist: ForiegnKey<Group>,
}

pub struct Album {
    id: Key,
    title: String,
    artist: ForiegnKey<Group>,
}

pub struct AlbumTag {
    album: ForiegnKey<Album>,
    tag: ForiegnKey<Tag>,
}

pub struct Group {
    id: Key,
    name: String,
}

pub struct GroupTag {
    group: ForiegnKey<Group>,
    tag: ForiegnKey<Tag>,
}

pub struct GroupMember {
    group: ForiegnKey<Group>,
    role: String,
    member: ForiegnKey<Group>,
}

pub struct TagCategory {
    id: Key,
    name: String,
}

pub struct Tag {
    id: Key,
    category: ForiegnKey<TagCategory>,
    value: String,
}
