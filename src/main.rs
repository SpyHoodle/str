use std::env;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
struct Item<'a> {
    name: &'a str,
    tags: &'a str,
}

impl<'a> Item<'a> {
    fn from(name: &'a str, tags: &'a str) -> Self {
        Self { name, tags }
    }
}

#[derive(Deserialize, Serialize)]
struct Area<'a> {
    name: &'a str,
    items: Vec<Item<'a>>,
}

impl<'a> Area<'a> {
    fn from(name: &'a str, items: Vec<Item<'a>>) -> Self {
        Self { name, items }
    }
}

impl<'a> PartialEq for Area<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Deserialize, Serialize)]
struct Storage<'a> {
    #[serde(default = "Storage::default_name")]
    name: &'a str,

    #[serde(default = "Storage::default_areas")]
    areas: Vec<Area<'a>>,
}

impl<'a> Storage<'a> {
    fn default_name() -> &'static str {
        "my_storage"
    }

    fn default_areas() -> Vec<Area<'a>> {
        vec![Area::from("0.0.0", vec![Item::from("empty", "none")])]
    }
}

impl<'a> Storage<'a> {
    // fn from(name: &'a str, areas: Vec<Area<'a>>) -> Self {
    //     Self { name, areas }
    // }

    fn store(&mut self, area: Area) {
        for item in &area.items {
            println!("\x1b[32m ✔️ Stored item \x1b[34m'{}'\x1b[32m to area \x1b[36m'{}'\x1b[32m with tags \x1b[33m'{}'\x1b[0m", item.name, area.name, item.tags);
        };

        if self.areas.contains(&area) {
            // TODO
        }
    }

    fn print(&self) {
        println!("\x1b[35m{}\x1b[0m", &self.name);
        for area in &self.areas {
            println!("\x1b[36m└> {}\x1b[0m", area.name);
            for item in &area.items {
                println!("    \x1b[34m└> {}\x1b[0m", item.name);
            }
        }
    }
}

fn usage() {
    println!("usage: str [action] [area] [item]");
}

fn main() {
    let mut storage: Storage = ron::from_str("()").unwrap();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let action = &args[1] as &str;

        match action {
            "store" => {
                if args.len() > 3 {
                    let name = &args[3..].join(" ") as &str;
                    let item = Item::from(name, "none");
                    let area = Area::from(&args[2] as &str, vec![item]);
                    storage.store(area);
                } else {
                    usage();
                };
            },
            "show" => {
                storage.print();
            },
            _ => usage(),
        };
    } else {
        usage();
    }
}
