use crate::index::Index;
use crate::object::{read_header, Object, ObjectBuffer, ObjectHeader, ObjectReadError, ObjectType};
use crate::object_reader::ObjectReader;
use crate::oid::ObjectId;
use crate::utils::remove_last;
use crate::Repository;
use std::collections::BTreeMap;
use std::{
    io,
    io::{BufRead, Read},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeEntry {
    pub name: String,
    pub oid: ObjectId,
    pub mode: u32,
}

/// In memory data representation of a git tree object
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tree {
    entries: Vec<TreeEntry>,
}

impl Tree {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    /// Returns an iterator to the entries of the tree
    pub fn iter(&self) -> impl Iterator<Item = &TreeEntry> {
        self.entries.iter()
    }
}

impl Default for Tree {
    fn default() -> Self {
        Tree::new()
    }
}

impl Object for Tree {
    fn to_buffer(&self) -> ObjectBuffer {
        // TODO: error handling
        use std::io::Write;

        let mut content = vec![];
        for entry in &self.entries {
            write!(&mut content, "{:o} {}\0", entry.mode, entry.name).unwrap();
            content.extend_from_slice(&entry.oid.0);
        }

        ObjectBuffer::new(ObjectType::Tree, &content)
    }
}

/// Read a tree object
pub fn read_tree_object(reader: &mut ObjectReader) -> Result<Tree, ObjectReadError> {
    let ObjectHeader { typ, .. } = read_header(reader)?;
    if typ != ObjectType::Tree {
        return Err(ObjectReadError::MismatchObjectType(ObjectType::Tree, typ));
    }
    read_tree_content(reader).map_err(|err| ObjectReadError::ContentReadError(err))
}

fn read_tree_content(reader: &mut ObjectReader) -> io::Result<Tree> {
    let mut buffer = vec![];

    let mut tree = Tree::new();
    loop {
        buffer.clear();
        let n = reader.read_until(0, &mut buffer)?;
        // EOF
        if n == 0 {
            break;
        }
        let output_str = std::str::from_utf8(remove_last(&buffer)).unwrap();
        if output_str.is_empty() {
            break;
        }

        // TODO: handle this gracefully
        let (mode, name) = output_str.split_once(' ').unwrap();

        // TODO: handle this gracefully
        let mode = u32::from_str_radix(mode, 8).unwrap();

        // hash
        let mut oid_buffer = [0u8; 20];
        reader.read_exact(&mut oid_buffer)?;
        let oid = ObjectId(oid_buffer);

        tree.entries.push(TreeEntry {
            name: name.to_string(),
            oid,
            mode,
        });
    }
    Ok(tree)
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TreeBuilderEntry {
    Blob(TreeEntry),
    Tree(TreeBuilder),
}

// Auxiliary data structure to build a tree from a flat list of entries
#[derive(Clone, Debug, PartialEq, Eq)]
struct TreeBuilder {
    entries: BTreeMap<String, TreeBuilderEntry>,
}

impl TreeBuilder {
    fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    fn add_entry(&mut self, path: &[String], name: String, entry: TreeEntry) {
        //println!("path: {} name: {}", path.join("/"), name);

        if path.is_empty() {
            // Add to the current tree

            assert_eq!(
                self.entries
                    .insert(name.clone(), TreeBuilderEntry::Blob(entry)),
                None,
            )
        } else {
            // Is a folder, add to child tree

            // Implementation note: git sort tree entries as if folder names contains a trailing /
            let folder_name = format!("{}/", path[0]);
            if let Some(TreeBuilderEntry::Tree(child_tree)) = self.entries.get_mut(&folder_name) {
                child_tree.add_entry(&path[1..], name, entry);
            } else {
                let mut child_tree = TreeBuilder::new();
                child_tree.add_entry(&path[1..], name, entry);
                self.entries
                    .insert(folder_name, TreeBuilderEntry::Tree(child_tree));
            }
        }
    }

    // returns the index to the root tree
    fn make_trees(self, trees: &mut Vec<Tree>) -> usize {
        let mut root = Tree::new();

        for (name, entry) in self.entries {
            match entry {
                TreeBuilderEntry::Blob(blob) => root.entries.push(blob),
                TreeBuilderEntry::Tree(child) => {
                    let child_index = child.make_trees(trees);
                    let buffer = trees[child_index].to_buffer();
                    let oid = ObjectId::from_object_buffer(&buffer);
                    root.entries.push(TreeEntry {
                        name: name[0..name.len() - 1].to_string(), /* remove the trailing slash */
                        oid,
                        mode: 0o040000,
                    })
                }
            }
        }

        let root_index = trees.len();
        trees.push(root);
        root_index
    }
}

impl Repository {
    /// Write the index as a tree. Including all the children trees. Returns the `ObjectId` of the root tree
    pub fn write_tree(&self) -> ObjectId {
        // TODO: error handling

        let index = Index::open(&self.git_dir.join("index")).unwrap();

        let entries: Vec<_> = index.iter().collect();

        let mut tree_builder = TreeBuilder::new();

        // Create in-memory trees
        for entry in &entries {
            let mut path: Vec<_> = entry
                .path
                .components()
                .map(|c| c.as_os_str().to_str().unwrap().to_string())
                .collect();

            let name = path
                .pop()
                .expect("Index entry path should have a file name");

            let oid = entry.oid;
            let mode = entry.metadata.mode;

            tree_builder.add_entry(&path, name.clone(), TreeEntry { name, oid, mode })
        }

        let mut trees = vec![];
        let root_index = tree_builder.make_trees(&mut trees);

        for tree in &trees {
            let tree_buffer = tree.to_buffer();
            let oid = ObjectId::from_object_buffer(&tree_buffer);
            self.write_object_buffer(oid, &tree_buffer).unwrap();
        }

        let root_buffer = trees[root_index].to_buffer();
        ObjectId::from_object_buffer(&root_buffer)
    }
}
