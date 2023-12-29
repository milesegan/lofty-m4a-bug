use std::{
    fs::{copy, remove_file},
    path::Path,
    process::Command,
};

use lofty::{Accessor, Probe, Tag, TagExt, TagType, TaggedFileExt};

pub fn write_tag(dest: &Path) {
    let mut dest_file = Probe::open(&dest)
        .expect("can't open")
        .read()
        .expect("can't read");
    let tag = Tag::new(TagType::Mp4Ilst);
    let mut dest_tag = dest_file.insert_tag(tag).expect("no tag");
    dest_tag.set_artist("test artist".to_string());
    dest_tag.save_to_path(dest).expect("can't write");
}

fn main() {
    let _ = remove_file("test.m4a");
    Command::new("afconvert")
        .args(["-d", "aac", "-f", "m4af", "test.flac", "encoded.m4a"])
        .output()
        .expect("couldn't encode file");
    println!("encoded to encoded.m4a");
    copy("encoded.m4a", "tagged.m4a").expect("copy failed");
    println!("copied to tagged.m4a");
    write_tag(Path::new("tagged.m4a"));
    println!("wrote tag to tagged.m4a");
}
