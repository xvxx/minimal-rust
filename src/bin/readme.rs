//! Generates README.md from README.tpl and lists/*.txt.

use std::fs;

const HEADER: &str = "| Crate | Desc | Dependencies† | Links |
| --- | --- | --- | --- |";

const ENTRY: &str = "| [`$crate`] | $desc | $deps | $links |";

const CRATE_LINK: &str = "[`$crate`]: https://crates.io/crates/$crate";

fn main() {
    let readme = fs::read_to_string("README.tpl").unwrap();
    let (table, footnote) = markdown();
    println!(
        "{}",
        readme
            .replace("$$LIST$$", &table)
            .replace("$$FOOTNOTE$$", &footnote)
    );
}

type Table = String;
type Footnote = String;

fn markdown() -> (Table, Footnote) {
    let mut out = String::new();
    let mut footnote = vec![];

    for deps in 0..3 {
        let path = format!("lists/{}.txt", deps);
        let list = fs::read_to_string(&path).unwrap();

        if deps == 1 {
            out.push_str("\n## One dependency\n\n");
        }

        if deps == 2 {
            out.push_str("\n## Two dependencies\n\n");
        }

        out.push_str(HEADER);
        out.push('\n');

        for (line_num, line) in list.split_terminator('\n').enumerate() {
            if matches!(line.chars().next(), Some('#')) {
                continue;
            }
            let mut parts = line.split_ascii_whitespace();
            let parts_len = parts.clone().count();

            if parts_len < 3 || !parts.clone().last().unwrap().starts_with("https://") {
                eprintln!(
                    "error at {} on line {}: {}\n{}",
                    path,
                    line_num + 1,
                    "format must be `CRATE DESC URL`",
                    line,
                );
                std::process::exit(1);
            }

            let name = parts.next().unwrap();
            let desc = parts
                .clone()
                .take(parts_len - 2)
                .collect::<Vec<_>>()
                .join(" ");
            let url = parts.skip(parts_len - 2).next().unwrap();
            let crate_links = format!(
                "[📦](https://crates.io/crates/{}) • [📚](https://docs.rs/{}) • [🏠]({})",
                name, name, url
            );

            out.push_str(&format!(
                "{}\n",
                ENTRY
                    .replace("$crate", name)
                    .replace("$desc", &desc)
                    .replace("$links", &crate_links)
                    .replace("$deps", &format!("{}", deps))
            ));
            footnote.push(CRATE_LINK.replace("$crate", name));
        }
    }

    (out, footnote.join("\n"))
}
