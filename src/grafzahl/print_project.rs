use std::path::PathBuf;

use quicli::prelude::error;
use structopt::StructOpt;

use crate::{Cli, AppState};
use crate::grafzahl::counter::Count;
use crate::grafzahl::tree_indexer::{File, Folder, FolderElement, get_name, scan_directory};
use crate::grafzahl::tree_indexer::FolderElement::*;

pub fn count_from_path(path: PathBuf, state: &mut AppState) {
    let can_path = path.canonicalize().expect(&*format!("Absolut Path could not be found for Path: {}", path.display()));
    let dir = scan_directory(&can_path, state);

    let args = Cli::from_args();
    if args.debug {
        println!("|| DEBUG ||");
        debug_structure(&dir, 0);
        println!("|| DEBUG ||");
        println!();
    }

    match dir {
        FolderEmpty => {
            println!("Project: {} ==> 0", get_name(&can_path));
            error!("No non empty Folder Could be found!");
            println!("  |- ----------------------------------");
            println!();
        }
        File(f) => {
            print_header(f.name.clone(),f.count);
            print_file(f,1)
        }
        Folder(f) => {
            print_header(f.name.clone(), f.total_count());
            if args.per_folder {
                print_dir_structure(f, 1);
            } else {
                print_dir_lang(f)
            }
        }
    }
    println!("|- ----------------------------------");
    println!();
}

fn debug_structure(ele: &FolderElement, indent_number: usize) {
    let indent = "  ".repeat(indent_number);
    match ele {
        Folder(f) => {
            println!("{}|- {} => ", indent, &f.name, );
            for member in &f.members {
                debug_structure(&member, indent_number + 1);
            }
        }
        File(f) => println!("{}|- {} =>", indent, f.name),
        FolderEmpty => println!("{}|- {} =>", indent, "--EMPTY--"),
    }
}

fn print_header(name: String, count: Count) {
    println!(
        "Project: {} ==> {}",
        name,
        (count.empty_count + count.code_count + count.comment_count)
    );
}

fn print_file(f: File, indent_number: usize) {
    let indent = "  ".repeat(indent_number);
    println!(
        "{}|- {} => {} (LoC: {}, Comment: {}, NewLines: {})",
        indent,
        f.name,
        (f.count.empty_count + f.count.code_count + f.count.comment_count),
        f.count.code_count,
        f.count.comment_count,
        f.count.empty_count
    );
}

fn print_dir_lang(dir: Folder) {
    for key in dir.count.keys() {
        let value = dir.count.get(key).unwrap();
        let lang_total = value.empty_count + value.code_count + value.comment_count;
        println!(
            "  |- {} => {} (LoC: {}, Comment: {}, NewLines: {})",
            key,
            lang_total,
            value.code_count,
            value.comment_count,
            value.empty_count
        );
    }
}

fn print_dir_structure(dir: Folder, indent_number: usize) {
    let indent = "  ".repeat(indent_number);
    println!(
        "{}|- {} => {}",
        indent,
        dir.name,
        dir.total()
    );
    for member in dir.members {
        match member {
            FolderEmpty => {}
            Folder(f) => print_dir_structure(f,indent_number+1),
            File(f) => print_file(f, indent_number+1),
        }
    }
}