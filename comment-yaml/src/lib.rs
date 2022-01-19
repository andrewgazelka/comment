use linked_hash_map::Entry;
use serde::Serialize;
pub use serde_yaml;
use serde_yaml::SerializerToYaml;
pub use yaml_rust as yaml;
use yaml_rust::yaml::{CommentedYaml, Comments};
use yaml_rust::Yaml;

use comment::Comment;

struct Ser<'a> {
    yaml: &'a mut Yaml,
}

pub fn to_yaml<S: Serialize + Comment>(value: &S) -> Yaml {
    let mut yaml = Serialize::serialize(&value, SerializerToYaml).unwrap();

    let ser = Ser { yaml: &mut yaml };

    // TODO: remove
    let _ = Comment::serialize(value, ser).unwrap();

    yaml
}

impl<'a> comment::CommentSerializer for Ser<'a> {
    type Ok = ();
    type Error = ();

    fn add_comment(&mut self, path: &str, _comment: &str) -> Result<(), Self::Error> {
        let key = Yaml::String(path.to_string());

        let hash = match self.yaml {
            Yaml::Hash(ref mut hash) => hash,
            _ => panic!("invalid"),
        };

        let mut occupied = match hash.entry(key) {
            Entry::Occupied(occupied) => occupied,
            Entry::Vacant(_) => panic!("should not happen"),
        };

        let from_val = occupied.get_mut();
        let from = core::mem::replace(from_val, Yaml::Null);

        let comments = Comments {
            before: vec![],
            head: vec![],
            line: None,
            tail: vec![],
            after: vec![],
        };
        let commented = CommentedYaml(Box::new(from), comments);

        *from_val = Yaml::CommentedYaml(commented);

        Ok(())
    }

    fn add_field<S: Comment>(&mut self, path: &str, value: &S) -> Result<(), Self::Error> {
        let key = Yaml::String(path.to_string());

        let hash = match self.yaml {
            Yaml::Hash(ref mut hash) => hash,
            _ => panic!("invalid"),
        };

        let child = hash.get_mut(&key).unwrap();
        let ser = Ser { yaml: child };
        value.serialize(ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
