use crate::oid::ObjectId;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeEntry {
    pub name: String,
    pub oid: ObjectId,
    pub mode: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeEntryRef<'a> {
    pub name: &'a str,
    pub oid: ObjectId,
    pub mode: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TreeEntryData {
    oid: ObjectId,
    mode: u32,
}

/// In memory data representation of a git tree object
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tree {
    entries: BTreeMap<String, TreeEntryData>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }
    pub fn add_entry(&mut self, entry: TreeEntry) {
        let TreeEntry { name, oid, mode } = entry;
        // TODO: how to handle duplicated name?
        assert_eq!(self.entries.insert(name, TreeEntryData { oid, mode }), None);
    }

    pub fn iter(&self) -> impl Iterator<Item = TreeEntryRef> {
        self.entries.iter().map(|(name, data)| TreeEntryRef {
            name,
            oid: data.oid,
            mode: data.mode,
        })
    }
}

impl Default for Tree {
    fn default() -> Self {
        Tree::new()
    }
}
