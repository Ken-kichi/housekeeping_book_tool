use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// 項目を追加する
    Add {
        #[structopt()]
        item: String,
        #[structopt()]
        money: usize,
        #[structopt()]
        desciption: String,
    },
    /// 項目を選択する
    Remove {
        #[structopt()]
        id: usize,
    },
    /// 項目を更新する
    Update {
        #[structopt()]
        id: usize,
        #[structopt()]
        item: String,
        #[structopt()]
        money: usize,
        #[structopt()]
        desciption: String,
    },
    /// 項目の一覧を表示する
    List,
    /// 選択した項目の詳細を表示する
    Detail {
        #[structopt()]
        id: usize,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Asset Management", about = "Rustで家計簿を記録するツール")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different file.
    #[structopt(parse(from_os_str), short, long)]
    pub asset_file: Option<PathBuf>,
}
