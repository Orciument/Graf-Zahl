use crate::grafv2::tree::{Count, DirectoryMember, FolderElement, LanguageCount, LocTotal};
use crate::grafv2::tree::FolderElement as FE;
use crate::grafv2::tree::FolderError::FailedRead;

pub(crate) fn print_structure_entry(member: DirectoryMember, override_ignore: bool) {
    let reverse_buffer: &mut Vec<String> = &mut Default::default();
    reverse_buffer.push("|- ----------------------------------".to_string());
    reverse_buffer.push("\n".to_string());

    let name = member.name.clone();

    let total = print_structure(member, reverse_buffer, override_ignore,0);

    println!("Project: {} ==> {}", name, total.total());
    for i in reverse_buffer.iter().rev() {
        println!("{}", i)
    }
}

fn print_structure(member: DirectoryMember, reverse_buffer: &mut Vec<String>, override_ignore: bool, depth: usize) -> Count {
    let indent = "  ".repeat(depth);
    // if member.ignored && !override_ignore {
    //     return Default::default();
    // }

    return match member.member {
        FolderElement::File(Ok(l)) => {
            reverse_buffer.push(format!("{}|- {} => {} ", indent, member.name, &l.count.total()));
            l.count
        }
        FolderElement::File(Err(e)) => {
            reverse_buffer.push(format!("{}|- {} => [ERR]: {} ", indent, member.name, e));
            Count::default()
        }
        FolderElement::Folder(Ok(memberList)) => {

            let mut total: Count = Default::default();
            for dm in memberList {
                total += print_structure(dm, reverse_buffer, override_ignore, depth + 1)
            }
            reverse_buffer.push(format!("{}|- {}/ => {} ", indent, member.name, &total.total()));
            total
        }
        FolderElement::Folder(Err(e)) => {
            reverse_buffer.push(format!("{}|- {}/ => [ERR]: {} ", indent, member.name, e));
            Count::default()
        }
    };
}

pub(crate) fn print_language_entry(member: DirectoryMember) {
    match member.member {
        FE::File(f) => {
            let count = if let Ok(ref v) = f {
                v.count.empty_count + v.count.comment_count + v.count.code_count
            } else { 0 };
            println!("Project: {} ==> {}", member.name, count);

            match f {
                Ok(lc) => print_lang(&lc),
                Err(err) => println!("{}", err),
            }
        }

        FE::Folder(Ok(ref v)) => {
            let langs = member.member.total_langs();
            let mut total: u32 = 0;
            for lang in &langs {
                total += lang.count.code_count + lang.count.comment_count + lang.count.empty_count;
            }
            println!(
                "Project: {} ==> {}",
                member.name,
                total
            );
            for lang in &langs {
                print_lang(lang);
            }
        }
        FE::Folder(Err(FailedRead(e))) => {
            println!("Project: {} ==> ???", member.name);
            println!("{}", e)
        }
    }
    println!("|- ----------------------------------");
    println!();
}

fn print_lang(lc: &LanguageCount) {
    let indent = "  ";
    println!(
        "{}|- {} => {} (LoC: {}, Comment: {}, NewLines: {})",
        indent,
        lc.name,
        (lc.count.empty_count + lc.count.code_count + lc.count.comment_count),
        lc.count.code_count,
        lc.count.comment_count,
        lc.count.empty_count
    );
}