use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn get_conf_dir() -> Result<PathBuf, Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Home directory doesn't exist")?;

    Ok(home_dir.join("eyecatch"))
}

pub fn verify() -> Result<(), Box<dyn Error>> {
    let conf_dir = get_conf_dir()?;
    fs::create_dir_all(&conf_dir)?;
    let eyecatch_file = conf_dir.join("TODO.json");

    if !eyecatch_file.exists() {
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .open(eyecatch_file)?;

        write!(f, "[]")?;
    }

    Ok(())
}

struct HexStyles<'a> {
    magenta: &'a str,
    yellow: &'a str,
    bold: &'a str,
    reset: &'a str,
}

const HEX_STYLES: HexStyles = HexStyles {
    magenta: "\x1B[35m",
    yellow: "\x1B[33m",
    bold: "\x1B[1m",
    reset: "\x1B[0m",
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: String,
    pub content: String,
}

impl Item {
    pub fn new(content: String, existing_ids: Vec<String>) -> Self {
        Self {
            id: Self::get_id(existing_ids),
            content,
        }
    }

    fn get_id(existing_ids: Vec<String>) -> String {
        let mut id = nanoid!(
            3,
            &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f']
        );

        while existing_ids.contains(&id) {
            id = nanoid!(
                3,
                &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f']
            );
        }

        id
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let injected = format!(
            "   {0}{1}{2}{3}  |  {4}",
            HEX_STYLES.magenta, HEX_STYLES.bold, self.id, HEX_STYLES.reset, self.content
        );

        write!(f, "{injected}")
    }
}

pub fn list(items: &[Item]) {
    if items.is_empty() {
        return;
    }

    let longest = items
        .iter()
        .max_by_key(|item| item.content.len())
        .unwrap()
        .content
        .len();

    let injected = format!(
        "\n{0}{1}   uid{2}  |  {0}{1}content{2}
+-------+--{3}--+",
        HEX_STYLES.yellow,
        HEX_STYLES.bold,
        HEX_STYLES.reset,
        "-".repeat(longest)
    );

    println!("{injected}");

    items.iter().for_each(|item| println!("{item}"));
    println!();
}

pub fn deserialize() -> Result<Vec<Item>, Box<dyn Error>> {
    let conf_dir = get_conf_dir()?;
    let eyecatch_file = conf_dir.join("TODO.json");

    let res = serde_json::from_str(&fs::read_to_string(eyecatch_file)?)?;

    Ok(res)
}

pub fn serialize(items: Vec<Item>) -> Result<(), Box<dyn Error>> {
    let res = serde_json::to_string_pretty(&items)?;

    let conf_dir = get_conf_dir()?;
    let eyecatch_file = conf_dir.join("TODO.json");

    let mut f = OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(eyecatch_file)?;

    write!(f, "{res}")?;

    Ok(())
}
