#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(non_snake_case)]

//TODO
// changes that should be made are not made,
// - [ ] fix alteration of the path file

use std::env;
use std::{fs, io::Write};

fn vec_to_string(v : &Vec<String>,start_index : i32) -> String{
    let mut res = String::new();
    for i in start_index as usize..v.len(){
        res += v[i].as_str();
        res +=  String::from(" ").as_str();
    }
    return res;
}

fn strike_out_element(ss : &String) -> String{
    let vec_version = split(&ss,' ');
    let item_sign = format!("{} {}x{}",vec_version[0],vec_version[1],vec_version[2]);
    return format!("{} {}",item_sign,vec_to_string(&vec_version,3));
}

fn split(ss : &String, split_char : char) -> Vec<String>{

    let mut res : Vec<String> = Vec::new();

    let mut temp = String::new();

    let as_str = ss.as_str();

    for element in as_str.chars() {
        if element == split_char{
            res.push(temp);
            temp = String::new();
        }
        else{
            temp.push(element);
        }
    }
    res.push(temp);

    return res;
}

fn strike_out(list : &mut Vec<String>,index : usize){
    if index < 0 as usize || index > list.len() as usize{
        return;
    }
    list[index] = strike_out_element(&list[index]);
}

fn write_todo_to_file(path : String, todos : Vec<String>){
    let mut file = fs::OpenOptions::new().write(true).truncate(true).open(path).expect("cant open file");
    let mut new_content = String::new();
    for i in todos {
        if i.len() != 0{
            new_content += &format!("{}\n",i).to_string();
        }
    }
    file.write_all(new_content.as_bytes()).expect("cant write to file");
}

fn add_to_vec_with_check(todos : &mut Vec<String>,new_item : String){
    todos.push(format!("- [ ] {}",new_item));
}

fn main() {
    let path = String::from("C:\\Users\\user\\Desktop\\volt\\todo.md");

    let args : Vec<String> = env::args().collect();

    let command = {
        if args.len() > 1{
            &args[1]
        }
        else{
            ""
        }
    };
    let rest = vec_to_string(&args,2);

    let content = fs::read_to_string(path.clone()).expect("no such file exists");
    let mut todos = split(&content,'\n');

    match command{
        "-s" => {
            let index = match split(&rest,' ')[0].parse::<usize>(){
                Ok(v) => v,
                Err(_)=> panic!()
            };
            strike_out(&mut todos,index as usize);
            write_todo_to_file(path,todos);
        },
        "-a" =>{
            add_to_vec_with_check(&mut todos,rest);
            write_todo_to_file(path,todos);
        },
        "-r" =>{
            let index = match split(&rest,' ')[0].parse::<usize>(){
                Ok(v) => v,
                Err(_)=> panic!()
            };
            todos.swap_remove(index);
            write_todo_to_file(path,todos);
        }
        _ => {
            for i in 0..todos.len() {
                println!("{}\t{}",i,todos[i]);
            }
        }
    }

}
