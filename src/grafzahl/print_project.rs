use std::path::PathBuf;

use quicli::prelude::error;

use crate::grafzahl::tree_indexer::{File, Folder, FolderElement, get_name, scan_directory};

pub fn count_from_path(path: PathBuf) {
    let can_path = path.canonicalize().expect(&*format!("Absolut Path could not be found for Path: {}", path.display()));
    let dir = scan_directory(&can_path);
    //TODO add debug Flag
    // debug_structure(&dir, 0);

    match dir {
        FolderElement::FolderEmpty => print_empty_toplevel(&can_path),
        FolderElement::File(f) => print_file(f),
        FolderElement::Folder(f) => print_directory(f)
    }
}

fn debug_structure(ele: &FolderElement, indent_number: usize) {
    let indent = "  ".repeat(indent_number);
    match ele {
        FolderElement::Folder(f) => {
            println!("{}|- {} => ", indent, &f.name,);
            for member in &f.members {
                debug_structure(&member, indent_number + 1);
            }
        }
        FolderElement::File(f) => println!("{}|- {} =>", indent, f.name),
        // FolderElement::FolderEmpty => println!("{}|- {} =>", indent, "--EMPTY--")

        _ => {}
    }
}

fn print_empty_toplevel(path: &PathBuf) {
    //Try to get current current Folder Name
    println!("Project: {} ==> 0", get_name(path));
    error!("No non empty Folder Could be found!");
    println!("  |- ----------------------------------");
    println!();
}

fn print_analysis(dir: FolderElement) {
    match dir {
        FolderElement::FolderEmpty => {}
        FolderElement::File(f) => print_file(f),
        FolderElement::Folder(f) => print_directory(f)
    }
}

fn print_file(f: File) {
    let total = (f.count.empty_count + f.count.code_count + f.count.comment_count);
    println!(
        "Project: {} ==> {}",
        f.name + &*f.extension,
        total
    );

    println!(
        "  |- {} => {} (LoC: {}, Comment: {}, NewLines: {})",
        f.language,
        total,
        f.count.code_count,
        f.count.comment_count,
        f.count.empty_count
    );
    println!("  |- ----------------------------------");
    println!();
}

fn print_directory(dir: Folder) {
    let mut total: u32 = 0;
    for value in dir.count.values() {
        total += value.empty_count + value.code_count + value.comment_count;
    }

    println!(
        "Project: {} ==> {}",
        dir.name,
        total
    );

    for key in dir.count.keys() {
        let value = dir.count.get(key).unwrap();
        let lang_total = (value.empty_count + value.code_count + value.comment_count);
        println!(
            "  |- {} => {} (LoC: {}, Comment: {}, NewLines: {})",
            key,
            lang_total,
            value.code_count,
            value.comment_count,
            value.empty_count
        );
    }
    println!("  |- ----------------------------------");
    println!();
}