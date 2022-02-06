use fpms::slot::do_slot;

use clap::Parser;
fn main() {
    #[derive(Parser)]
    #[clap(version, about, long_about = None)]
    struct Cli {
        /// chmod 対象のファイルパス
        #[clap(short, long)]
        file: String,
    }
    let args = Cli::parse();
    do_slot(&args.file);
}
