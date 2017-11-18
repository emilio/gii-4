#[macro_use]
extern crate clap;

use std::io;
use std::rc::Rc;
use std::collections::HashMap;
use std::path::Path as StdPath;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct EdgeId(Rc<String>);

impl EdgeId {
    pub fn new<T: Into<String>>(name: T) -> Self {
        EdgeId(Rc::new(name.into()))
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct ItemId(Rc<String>);

impl ItemId {
    pub fn new<T: Into<String>>(name: T) -> Self {
        ItemId(Rc::new(name.into()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Path {
    to: EdgeId,
    distance: u32,
}

#[derive(Debug, Default)]
struct Map {
    /// A map from an edge to all the paths that you can take from it.
    graph: HashMap<EdgeId, Vec<Path>>,
    /// A map from an edge to the storage, which can contain items, and for each
    /// item the amount of items of that kind that exist.
    storage: HashMap<EdgeId, HashMap<ItemId, u32>>,
}

impl Map {
    pub fn from_file(file_name: &StdPath) -> Result<Self, io::Error> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = BufReader::new(File::open(file_name)?);
        let mut map = Map::default();

        for (i, line) in file.lines().enumerate() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('%') {
                continue;
            }

            // Pretty crappy parser for what we need.
            let opening_paren = match line.find('(') {
                Some(p) => p,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Line {} wasn't a function call: {:?}", i, line),
                    ));
                }
            };

            let closing_paren = match line[opening_paren..].find(')') {
                Some(p) => opening_paren + p,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Line {} wasn't a function call: {:?}", i, line),
                    ));
                }
            };

            let function_name = &line[..opening_paren];
            let args = line[opening_paren + 1..closing_paren]
                .split(',')
                .map(|s| s.trim());

            match function_name {
                "conectado" | "ubicacion" => {},
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "Unknown function name at line {}: {:?}",
                            i,
                            function_name,
                        )
                    ));
                }
            };

            println!("{:?}, {:?}", function_name, args.collect::<Vec<_>>());
            // TODO.
        }

        Ok(map)
    }
}

fn main() {
    let matches = app_from_crate!()
        .args_from_usage(
            "<input>             'File where the prolog statements that define \
                                  the map are contained'
             -s, --start=[start] 'The start edge, defaults to \"S\"'
             -e, --end=[end]     'The end edge, defaults to \"T\"'",
        )
        .get_matches();

    let input = StdPath::new(matches.value_of("input").unwrap());
    let start = EdgeId::new(matches.value_of("start").unwrap_or("S"));
    let end = EdgeId::new(matches.value_of("end").unwrap_or("T"));

    let map = Map::from_file(&input).expect("Couldn't read map");

    println!("{}, {:?}, {:?}", input.display(), start, end);
    println!("{:?}", map);
}
