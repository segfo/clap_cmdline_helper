# このリポジトリの使い方
Cargo.tomlに以下のように依存関係を記述します。（cmdline_helper_types）
```
[dependencies]
clap = {version="4.5.4",features=["derive"]}
cmdline_helper={git="https://github.com/segfo/clap_cmdline_helper"}
cmdline_helper_types={git="https://github.com/segfo/clap_cmdline_helper_types"}
```

```
use clap::{CommandFactory, Parser};
use cmdline_helper_types::*;
use std::ffi::OsString;

use cmdline_helper::*;
// -z 引数で、バージョンを表示するように指示する例です。
#[helpargs_perser(version='z')]
#[derive(Parser, Debug, HelpArgs)]
#[command(author, version, about, long_about = None,disable_help_flag=true,disable_version_flag=true)]
struct HogeCommand {
    args:String,
    args2:Option<String>
}

fn main() {
    // $ command -z
    let cmd = vec!["command", "-z"];
    match HogeCommand::try_parse_from_iter(&cmd) {
        // CmdlineResult::Ok(cmd)型が返却される場合は、引数がすべて問題なくパースされ
        // HogeCommandのインスタンスとして生成されています。
        CmdlineResult::Ok(cmd) => {
            /* Fooには引数で指定された値が正しくパースされて挿入されています。
            構造体のメンバを利用し処理を進めます。 */
        }
        // CmdlineResult::Msg(msg,ty)型が返却される場合は以下の通りです。
        // ヘルプ・バージョンを表示するようなオプションが指定されたときはmsgに入ります。
        // パースエラーが発生したときも同様ですが、パースエラーであるかの特定もtyを利用すれば可能です。
        CmdlineResult::Msg(msg, ty) => {
            match ty{
                CmdlineMsgHint::PerseErrorHelp=>{println!("パースエラーが発生しました。");},
                _=>{println!("{}", msg);}
            }
        }
    }
}

```