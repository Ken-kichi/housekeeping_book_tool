use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: usize,
    pub item: String,
    pub money: usize,
    pub description: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub update_at: DateTime<Utc>,
}

impl Item {
    pub fn new(item: String, money: usize, description: String) -> Item {
        let created_at: DateTime<Utc> = Utc::now();
        let update_at: DateTime<Utc> = Utc::now();
        Item {
            id: 0,
            item,
            money,
            description,
            created_at,
            update_at,
        }
    }
}

pub fn collect_items(mut file: &File) -> Result<Vec<Item>> {
    // ファイルのカーソルを先頭に移動
    file.seek(SeekFrom::Start(0))?;
    let items = match serde_json::from_reader(file) {
        Ok(items) => items,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(items)
}

pub fn add_item(asset_path: PathBuf, mut item: Item) -> Result<()> {
    // ファイルを開く
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(asset_path)?;

    // ファイルに接続する処理
    let mut items = collect_items(&file)?;

    let new_id = items.len() + 1;
    item.id = new_id;

    items.push(item);
    serde_json::to_writer(&file, &items)?;

    Ok(())
}

pub fn remove_item(asset_path: PathBuf, id: usize) -> Result<()> {
    let file = OpenOptions::new().read(true).write(true).open(asset_path)?;

    let mut items = collect_items(&file)?;

    if id == 0 || id > items.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Item ID"));
    }
    items.remove(id - 1);

    // ファイルサイズを0にしてから書き込む
    file.set_len(0)?;
    serde_json::to_writer(&file, &items)?;

    Ok(())
}

pub fn update_item(
    asset_path: PathBuf,
    id: usize,
    update_item: String,
    update_money: usize,
    update_desciption: String,
) -> Result<()> {
    let file = OpenOptions::new().read(true).write(true).open(asset_path)?;

    let items = collect_items(&file)?;

    let updated_items: Vec<Item> = items
        .iter()
        .map(|x| {
            if x.id == id {
                Some(Item {
                    id,
                    item: update_item.clone(),
                    money: update_money,
                    description: update_desciption.clone(),
                    created_at: x.created_at,
                    update_at: Utc::now(),
                })
            } else {
                Some(Item {
                    id: x.id,
                    item: x.item.clone(),
                    money: x.money,
                    description: x.description.clone(),
                    created_at: x.created_at,
                    update_at: x.update_at,
                })
            }
        })
        .flatten()
        .collect();

    let _ = file.set_len(0);
    serde_json::to_writer(&file, &updated_items)?;

    Ok(())
}

pub fn list_items(asset_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(asset_path)?;

    let items = collect_items(&file)?;

    println!(
        "{:<5} {:<20} {:<20}",
        "ID".to_string(),
        "項目".to_string(),
        "金額".to_string()
    );
    if items.is_empty() {
        println!("項目がありません!")
    } else {
        for item in items {
            println!("{:<5} {:<20} {:<20}", item.id, item.item, item.money,);
        }
    }

    Ok(())
}

pub fn detail_item(asset_path: PathBuf, id: usize) -> Result<()> {
    let file = OpenOptions::new().read(true).open(asset_path)?;

    let items = collect_items(&file)?;

    let selected_item: Vec<_> = items.iter().filter(|x| x.id == id).collect();

    println!(
        "{:<5} {:<20} {:<20} {:<20} {:<20} {:<20}",
        "ID".to_string(),
        "項目".to_string(),
        "金額".to_string(),
        "説明".to_string(),
        "作成日".to_string(),
        "更新日".to_string(),
    );

    println!(
        "{:<5} {:<20} {:<20} {:<20} {:<20} {:<20}",
        selected_item[0].id,
        selected_item[0].item,
        selected_item[0].money,
        selected_item[0].description,
        selected_item[0].created_at,
        selected_item[0].update_at
    );

    Ok(())
}
