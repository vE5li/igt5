extern crate dst6 as kami;

mod checked;

use std::path::Path;
use std::env;

use kami::*;
use checked::Checked;

fn print_help_menu() {
    println!("usage:");
    println!("   $   igt5 [options]");
    println!("options:");
    println!("  -h   show help menu");
    println!("  -a   pass compiler arguments");
    println!("  -d   set working directory");
    println!("  -p   change project file");
    println!("contact:");
    println!("   #   github.com/ve5li/igt5");
    println!("   @   ve5li@tuta.io");
}

fn main() {

    initialize_time();

    let mut command_line: Vec<String> = env::args().skip(1).collect();
    let mut working_directory = Checked::none("working directory");
    let mut project_file = Checked::some("project file", String::from("compiler.data"));
    let mut show_help_menu = false;

    while !command_line.is_empty() {
        match command_line.remove(0).as_ref() {
            "-a" => break,
            "-h" => show_help_menu = true,
            "-d" => display!(working_directory.update(&mut command_line)),
            "-p" => display!(project_file.update(&mut command_line)),
            invalid => display!(error!(Message, string!("unknown flag {}", invalid))),
        }
    }

    if show_help_menu {
        print_help_menu();
        return;
    }

    if let Some(working_directory) = working_directory.changed() {
        let status = env::set_current_dir(&Path::new(&working_directory));
        if status.is_err() {
            let message = format!("failed to set working directory {}", working_directory);
            display!(error!(Message, string!(message.as_str())));
        }
    }

    let parameters: SharedVector<Data> = command_line.iter().map(|argument| string!(argument.as_str())).collect();
    let project_file = project_file.into_inner().unwrap();
    let root = display!(read_map(&SharedString::from(&project_file)));
    let build = map!();

    let main_function_path = path!(vector![keyword!("function"), keyword!("main")]);
    match display!(root.index(&main_function_path), &Some(&root), &build) {
        Some(main_function) => display!(function(&main_function, parameters, &None, &root, &build), &Some(&root), &build),
        None => display!(error!(Message, string!("main function not found")), &Some(&root), &build),
    };
}
