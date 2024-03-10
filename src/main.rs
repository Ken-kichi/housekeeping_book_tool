mod cli;
use structopt::StructOpt;
mod item;

use cli::{Action::*, CommandLineArgs};
use item::Item;

fn main() {
    let CommandLineArgs { action, asset_file } = CommandLineArgs::from_args();

    let asset_file = asset_file.expect("ファイルの読み取りに失敗しました。");

    match action {
        Add {
            item,
            money,
            desciption,
        } => item::add_item(asset_file, Item::new(item, money, desciption)),
        List => item::list_items(asset_file),
        Remove { id } => item::remove_item(asset_file, id),
        Update {
            id,
            item,
            money,
            desciption,
        } => item::update_item(asset_file, id, item, money, desciption),
        Detail { id } => item::detail_item(asset_file, id),
    }
    .expect("アクションできませんでした。")
}
