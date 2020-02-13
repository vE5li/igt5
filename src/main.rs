#![feature(allocator_api)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod internal;
mod tokenize;
mod parse;
mod build;

use internal::*;
use std::path::Path;
use std::env;

fn print_help_menu() {
    println!("usage:");
    println!("   $   igt5 <method> [options]");
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
    let mut project_file = Checked::some("project file", String::from("project"));
    let mut show_help_menu = false;
    let context = map!();
    let build = map!();

    while !command_line.is_empty() {
        match command_line.remove(0).as_ref() {
            "-a" => break,
            "-h" => show_help_menu = true,
            "-d" => display!(working_directory.update(&mut command_line), &None, &build, &context),
            "-p" => display!(project_file.update(&mut command_line), &None, &build, &context),
            invalid => display!(error!(Message, string!(str, "unknown flag {}", invalid)), &None, &build, &context),
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
            display!(error!(Message, string!(str, message.as_str())), &None, &build, &context);
        }
    }

    // ensure all parameters are ascii
    let parameters: Vector<Data> = command_line.iter().map(|argument| string!(str, argument.as_str())).collect();
    let project_file = project_file.into_inner().unwrap();
    let root = display!(read_map(&AsciiString::from(&project_file)), &None, &build, &context);

    let main_method_path = path!(vector![keyword!(str, "method"), keyword!(str, "main")]);
    match display!(root.index(&main_method_path), &Some(&root), &build, &context) {
        Some(main_method) => display!(method(&main_method, parameters, &None, &root, &build, &context), &Some(&root), &build, &context),
        None => display!(error!(Message, string!(str, "main method not found")), &Some(&root), &build, &context),
    };
}
