use std::collections::HashMap;
use std::path::PathBuf;

use crate::grafzahl::file_filter::filter_files;
use crate::grafzahl::language_counter::{count_project_files, LineData};
use crate::grafzahl::package_indexer::search_files;

pub struct ProjectLangs {
    project_dir_name: String,
    total: LineData,
    data: HashMap<&'static str, LineData>,
}

pub fn analyse_project(path: PathBuf) -> Option<ProjectLangs> {
    let dir_name = path.file_name()?.to_str()?.to_string();
    //Index
    let mut vec = search_files(path).unwrap();
    //Filter
    filter_files(&mut vec);
    //Count
    let lang_map: HashMap<&str, LineData> = count_project_files(vec);

    //Add Totals
    let sum = lang_map
        .values()
        .fold((0, 0, 0), |sum: (u32, u32, u32), x| {
            (
                sum.0 + x.code_count,
                sum.1 + x.comment_count,
                sum.2 + x.empty_count,
            )
        });

    Some(ProjectLangs {
        project_dir_name: dir_name,
        total: LineData {
            comment_count: sum.0,
            code_count: sum.1,
            empty_count: sum.2,
        },
        data: lang_map,
    })
}

pub fn display_project(p: ProjectLangs) {
    println!(
        "Project: {} ==> {}",
        p.project_dir_name,
        (p.total.comment_count + p.total.code_count + p.total.empty_count)
    );
    for key in p.data.keys() {
        let value = p.data.get(key).unwrap();
        println!(
            "  |- {} => {} (LoC: {}, Comment: {}, NewLines: {})",
            key,
            (value.code_count + value.comment_count + value.empty_count),
            value.code_count,
            value.comment_count,
            value.empty_count
        );
    }
    println!("  |- ----------------------------------");
    println!();
}
