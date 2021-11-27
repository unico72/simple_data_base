use std::fs;
use std::fs::File;
use std::io::prelude::*;

struct DataBase {
    table_name: String,
    colums: Vec<(String, i8)>,
    data: Vec<String>
}

impl DataBase {
    fn print_struct(&self) {
        println!("table_name: '{}'\ncolums: '{:?}'", self.table_name, self.colums);
    }
    fn return_(&self, param: Vec<String>, vals: Vec<String>) -> Vec<String> {
        let mut ret = Vec::new();    
        let mut param_index: Vec<usize> = Vec::new();
        for i in 0..self.colums.len() {
            for j in 0..param.len() {
                if self.colums[i].0.clone() == param[j].clone() {
                    param_index.push(i);
                }
            }
        }
        for line in self.data.clone() {            
            let temp_: Vec<&str> = line.split('\t').collect::<Vec<&str>>();
            for i in 0..temp_.len() {
                for j in 0..param_index.len() {
                    if i == param_index[j] {
                        for v in 0..vals.len() {
                            if temp_[i].clone() == vals[v].clone().as_str() {
                                ret.push(line.clone())
                            }
                        }
                    }
                }
            }            
        }
        ret
    }
    fn add(&mut self, param: Vec<String>, mut vals: Vec<String>) {
        let mut param_index: Vec<usize> = Vec::new();
        for i in 0..self.colums.len() {
            for j in 0..param.len() {
                if self.colums[i].0.clone() == param[j].clone() {
                    param_index.push(j);
                }
            }
        }
        //println!("{:?}", param_index);
        for i in 0..param_index.len() {
            for j in 0..param_index.len() {
                let temp_i = param_index[i].clone();
                let temp_j = param_index[j].clone();
                if temp_i < temp_j {
                    let temp: usize = param_index[j];
                    param_index[j] = param_index[i];
                    param_index[i] = temp;

                    let vals_: String = vals[j].clone();
                    vals[j] = vals[i].clone();
                    vals[i] = vals_.clone();
                }
            }
        }
        //println!("{:?}", vals);
        let mut val: String = String::new();
        for i in 0..vals.len() {
            val += vals[i].as_str().clone();
            if (i + 1) < vals.len() { 
                val.push('\t');
            }
        }
        self.data.push(val)
    }
    fn save(&self) {
        /*
            let mut file = File::create("foo.txt")?;
            file.write_all(b"Hello, world!")?;
            fs::write("bar.txt", "dolor sit")?; // Эта функция создаст файл, если он не существует, и полностью заменит его содержимое, если он существует.
        */
        let mut file: String = self.table_name.clone();
        file += ".db";
        println!("{:?}", file);
        let mut datas: String = String::new();
        
        datas += "===TABLE=== ";
        datas += self.table_name.as_str().clone();     
        
        for col in self.colums.clone() {
            datas.push('\n');
            datas += "==COL== ";
            datas += col.0.as_str();
            datas.push(' ');
            datas += col.1.to_string().as_str();
            //colums: Vec<(String, i8)>,
        }
        datas.push('\n');
        datas += "==DATA==";
        for d in self.data.clone() {
            datas.push('\n');
            datas += d.as_str();            
        }
        println!("{}", datas);
        fs::write(file.clone(), datas.clone()).unwrap();
    }
    fn remove(&mut self, param: Vec<String>, vals: Vec<String>) {
        let mut param_index: Vec<usize> = Vec::new();
        for i in 0..self.colums.len() {
            for j in 0..param.len() {
                if self.colums[i].0.clone() == param[j].clone() {
                    param_index.push(i);
                }
            }
        }
        let tmp_data = self.data.clone();
        for k in 0..tmp_data.len() {            
            let temp_: Vec<&str> = tmp_data[k].split('\t').collect::<Vec<&str>>();
            for i in 0..temp_.len() {
                for j in 0..param_index.len() {
                    if i == param_index[j] {
                        for v in 0..vals.len() {
                            if temp_[i].clone() == vals[v].clone().as_str() {
                                self.data.remove(k);
                            }
                        }
                    }
                }
            }            
        }
    }
    fn empty() -> DataBase {
        DataBase { table_name: String::new(), colums: Vec::new(), data: Vec::new() }
    }
    fn mut_clone(&mut self, db: DataBase) {
        self.table_name = db.table_name; self.colums = db.colums; self.data = db.data;
    }
}


fn create_db(struct_db: String) -> DataBase {
    //create tablename: col1 INT, col2 TEXT, col3 BOOL
    let name: String = struct_db.clone().split(':').collect::<Vec<&str>>()[0].to_string()
                                .split(' ').collect::<Vec<&str>>().iter().map(|&x| String::from(x)).collect::<Vec<String>>()[0].clone();
    let colums_: Vec<String> = struct_db.clone().split(':').collect::<Vec<&str>>()[1].to_string().split(',')
                                .collect::<Vec<&str>>().iter().map(|&x| String::from(x)).collect::<Vec<String>>();
    let mut ret: Vec<(String, i8)> = Vec::new();
    for i in 0..colums_.len() {
        // [ col1 type ]
        let line: String = colums_[i].clone();
        let arg_: Vec<&str> = colums_[i].split(' ').collect::<Vec<&str>>().clone();
        let mut name: String = "".to_string();
        let mut typ: i8 = 0;
        for j in 0..arg_.len() {
            if arg_[j] != "" &&  name == "".to_string() {
                name = arg_[j].to_string().clone();
            } else if arg_[j] != "" &&  name != "".to_string() {
                typ = match arg_[j].clone().trim() {
                    "INT" => 1,
                    "TEXT" => 2,
                    "BOOL" => 3,
                    _ => 2,
                    //INT, TEXT, BOOL
                };                                
                ret.push((name.clone(), typ.clone()));
            }
        }        
    }
    DataBase { table_name: name.clone(), colums: ret.clone(), data: Vec::new() }
}

fn open_db(path: String) -> String { 
    let mut file = match File::open(path.as_str()) {
        Ok(a) => a,
        Err(e) => { panic!("{}", e); File::create("foo.txt").unwrap() },
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("file is empty (6 line opendb)");
    contents
}

fn connect_db(contents: String) -> DataBase {
    let text: Vec<String> = contents.split('\n').collect::<Vec<&str>>().iter().map(|&x| String::from(x)).collect::<Vec<String>>();
    let mut tablename: String = String::new();
    let mut colums: Vec<(String, i8)> = Vec::new();
    let mut datas: bool = false;
    let mut data: Vec<String> = Vec::new();
    for i in 0..text.len() {
        match i {
            0 => { tablename = text[0].clone().split(' ').collect::<Vec<&str>>()[1].to_string() },
            _ => { 
                if !datas {
                    if text[i].trim() == "==DATA==" {
                        datas = true;
                    } else {
                        let mut content_: Vec<&str> = text[i].split(' ').collect::<Vec<&str>>();
                        if content_[0].clone() != "==COL==" {
                            panic!("Its DB crashed.// 15line read/ {:?}", text[i].clone());
                        }
                        let colname: String = content_[1].clone().to_string();
                        let type_: i8 = match content_[2].clone().trim().to_string().parse::<i8>() {
                                Ok(a) => a,
                                Err(e) => { panic!("{}\n\n '{:?}'", e, content_.clone()); 9 },
                            };
                        colums.push((colname.clone(), type_.clone()));
                    }
                } else {
                    data.push(text[i].trim().clone().to_string());
                }
            }
        }
    }
    //println!("table: {}\n col: {:?}\n\n data: {:?}", tablename, colums, data);
    DataBase { table_name: tablename, colums: colums, data: data, }
}


fn cmd(cmd: String, db: &mut DataBase) {
    match cmd.clone().split(' ').collect::<Vec<&str>>()[0] {
        "open" => {
            let mut con: String = String::new();
            let cmd_d = cmd.split(' ').collect::<Vec<&str>>();
            for i in 1..cmd_d.len() {
                con += cmd_d[i].clone();
                if (i + 1) < cmd_d.len() {
                    con.push(' ');
                } 
            }
            println!("open: '{}'", con);
            db.mut_clone(connect_db(open_db(con.clone())));
        }, 
        "add" => {
            let mut con: String = String::new();
            let cmd_d = cmd.split(' ').collect::<Vec<&str>>();
            for i in 1..cmd_d.len() {
                con += cmd_d[i].clone();
                if (i + 1) < cmd_d.len() {
                    con.push(' ');
                } 
            }
            //let mut tuples: Vec<(String, String)> = Vec::new();
            let mut column_name_to_add: Vec<String> = Vec::new();
            let mut column_val_to_add:  Vec<String> = Vec::new();
            let mut column: String = String::new();
            let mut value: String = String::new();
            let mut bracets: bool = false;
            let mut comma: bool = false;
            let mut brup: bool = false;
            for ch in con.chars(){
                if ch == '(' {
                    bracets = true;
                    continue;
                } else if ch == ')' {
                    bracets = false;
                    comma = false;
                    brup = false;
                    column_name_to_add.push(column.clone());
                    column_val_to_add.push(value.clone());
                    column = String::new();
                    value  = String::new();
                    continue;
                }
                if ch == ',' {
                    comma = true;
                    continue;
                }
                if ch == ' ' && !brup {
                    continue;
                } 
                if bracets && !comma {
                    column.push(ch.clone());
                }
                if bracets && comma && ch == '\'' {
                    brup = true;
                    continue;
                }
                if bracets && comma && ch != '\'' {
                    value.push(ch.clone());
                }
                if bracets && comma && brup && ch == '\'' {
                    brup = false;
                }
            }
            println!("add: {:?}, {:?}", column_name_to_add, column_val_to_add);
            db.add(column_name_to_add, column_val_to_add);  
        },
        "save" => {
            db.save();
        },
        "remove" => {
            let mut con: String = String::new();
            let cmd_d = cmd.split(' ').collect::<Vec<&str>>();
            for i in 1..cmd_d.len() {
                con += cmd_d[i].clone();
                if (i + 1) < cmd_d.len() {
                    con.push(' ');
                } 
            }
            //let mut tuples: Vec<(String, String)> = Vec::new();
            let mut column_name_to_add: Vec<String> = Vec::new();
            let mut column_val_to_add:  Vec<String> = Vec::new();
            let mut column: String = String::new();
            let mut value: String = String::new();
            let mut bracets: bool = false;
            let mut comma: bool = false;
            let mut brup: bool = false;
            for ch in con.chars(){
                if ch == '(' {
                    bracets = true;
                    continue;
                } else if ch == ')' {
                    bracets = false;
                    comma = false;
                    brup = false;
                    column_name_to_add.push(column.clone());
                    column_val_to_add.push(value.clone());
                    column = String::new();
                    value  = String::new();
                    continue;
                }
                if ch == ',' {
                    comma = true;
                    continue;
                }
                if ch == ' ' && !brup {
                    continue;
                } 
                if bracets && !comma {
                    column.push(ch.clone());
                }
                if bracets && comma && ch == '\'' {
                    brup = true;
                    continue;
                }
                if bracets && comma && ch != '\'' {
                    value.push(ch.clone());
                }
                if bracets && comma && brup && ch == '\'' {
                    brup = false;
                }
            }
            println!("remove: {:?}, {:?}", column_name_to_add, column_val_to_add);
            //db.add(column_name_to_add, column_val_to_add);  
            db.remove(column_name_to_add, column_val_to_add);
        },
        _ => {  },
    }    
}

fn main () {
    /*connect_db(open_db("database.db".to_string()));
    let db = DataBase { table_name: "tb".to_string(), colums: vec![("one".to_string(), 1), ("two".to_string(), 2), ("three".to_string(), 3)], data: vec!["1\t1\t1".to_string(),
    "2\t2\t2".to_string(), "3\t3\t3".to_string(), "4\t4\t4".to_string()], };
    println!("{:?}", db.return_(vec!["one".to_string()], vec!["3".to_string()]));
    println!("\n\n\n####\n\n");*/
    //create tablename: col1 INT, col2 TEXT, col3 BOOL
    let mut db: DataBase = DataBase::empty();
    cmd("open user.db".to_string(), &mut db);
    cmd("remove (login, 'admin1')".to_string(), &mut db);
    //cmd("save".to_string(), &mut db);
    println!("{}", db.return_(vec!["login".to_string()], vec!["admin1".to_string()])[0]);
    /*
    let mut db = create_db("user: id INT, login TEXT, gender BOOL".to_string());
    //db.print_struct();
    db.add(vec!["login".to_string(),"id".to_string(),"gender".to_string()], vec!["admin".to_string(),"0".to_string(),"FALSE".to_string()]);
    db.add(vec!["login".to_string(),"id".to_string(),"gender".to_string()], vec!["admin1".to_string(),"1".to_string(),"TRUE".to_string()]);
    db.add(vec!["login".to_string(),"id".to_string(),"gender".to_string()], vec!["admin2".to_string(),"2".to_string(),"FALSE".to_string()]);
    db.add(vec!["login".to_string(),"id".to_string(),"gender".to_string()], vec!["admin3".to_string(),"3".to_string(),"TRUE".to_string()]);

    db.remove(vec!["login".to_string()], vec!["admin2".to_string()]);

    println!("{}", db.return_(vec!["login".to_string()], vec!["admin".to_string()])[0]);
    db.save();
    */

    //fn add(&mut self, param: Vec<String>, mut vals: Vec<String>) {    
}

/*

    ===TABLE=== tablename
    ==COL== columnname type
    ==COL== columnname type
    ==COL== columnname type
    ==COL== columnname type
    ==DATA==
    •••
    data \t data \t data \t
    •••
    
*/

/*
    String - 0
    int - 1
    real - 2
*/