mod find;
use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use colored::*;

fn main() {
    let args : Vec<String> = env::args().collect();
    //参数1：搜索目录；参数2；要搜索的正则表达式
    if args.len() < 3 {
        eprintln!("使用方式：{} <-v(可选择)> <目标目录数量> <目录1> <目录2>... <正则表达式数量> <表达式1> <表达式2>...",args[0]);
        process::exit(1);
    }
    //思考一下，如果用户输入的参数太多，应该怎么样？
    let mut path_num : usize = 0;
    let mut string_num : usize = 0;
    let mut flag = 0;
    if args[1] == "-v" {
        flag = 1;
    }
    path_num = (&args[flag + 1]).parse::<usize>().unwrap();
    string_num = (&args[flag + 2 + path_num]).parse::<usize>().unwrap();


    let mut pattern = &args[flag + 2 + path_num];
    let mut file_path = &args[flag + 1];
    for i in 1..=path_num {
        for j in 1..=string_num {
            pattern = &args[flag + 2 + path_num + j];
            file_path = &args[flag + 1 + i];
            let regex = match Regex::new(pattern) {
                Ok(re) => re,
                Err(err) => {
                    eprintln!("无效的正则表达式 '{}': {}", pattern, err);
                    process::exit(1);
                }
            };
            match find::find(&file_path, &regex) {
                Ok(matches) => {
                    if matches.is_empty() {
                        println!("未找到匹配项。");
                    } else {
                        println!("在目录“ {} ”中匹配表达式“ {} ”,找到以下匹配项：\n", file_path.bright_green(), pattern.bright_green());
                        for file in matches {
                            if flag == 1 {
                                println!("{},内容如下：", file.yellow());
                                let file_content = File::open(file).unwrap();
                                let reader = BufReader::new(file_content);
                                for line in reader.lines() {
                                    let line = line.unwrap();
                                    println!("{}", line);
                                }
                            }
                            else if flag == 0 {
                                println!("{}", file.yellow());
                            }
                            println!("");
                        }
                    }
                }
                Err(error) => {
                    eprintln!("发生错误：{}", error);
                    process::exit(1);
                }
            }
        }
        

    }
}
