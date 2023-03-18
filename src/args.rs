use crate::commands::{decode, encode, print_chunks, remove};
use clap::{arg, Command};

pub fn cli() {
    let matches = Command::new("pngme")
        .subcommands([
            Command::new("encode")
                .arg(arg!(<PATH>))
                .arg(arg!(<CHUNK_TYPE>))
                .arg(arg!(<MESSAGE>))
                .arg(arg!([output])),
            Command::new("decode")
                .arg(arg!(<PATH>))
                .arg(arg!(<CHUNK_TYPE>)),
            Command::new("remove")
                .arg(arg!(<PATH>))
                .arg(arg!(<CHUNK_TYPE>)),
            Command::new("print").arg(arg!(<PATH>)),
        ])
        .get_matches();

    match matches.subcommand() {
        Some(("encode", subcommands)) => encode(subcommands),
        Some(("decode", subcommands)) => decode(subcommands),
        Some(("remove", subcommands)) => remove(subcommands),
        Some(("print", subcommands)) => print_chunks(subcommands),
        _ => (),
    }
}
