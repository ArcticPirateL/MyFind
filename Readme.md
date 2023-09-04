# Rust HW1：MyFind



3210104927    刘子鸣



## 1. 将代码重构到多个模块



- 将原本代码中的`find`函数和`walk_tree`函数迁移到与`main.rs`同目录下的新文件中，命名为“`find.rs`”。

  ![image-20230904211829923](/images/image-20230904211829923.png)

- 对应地，在`main.rs`代码起始添加`mod find;`语句。

- 在`main.rs`函数中调用的`find`函数前加上“`find::`”，声明其来源。

  ```rust
  match find::find(&file_path, &regex) {}
  ```

- 在`find.rs`文件中的函数前加上`pub`关键字。

  ```rust
  pub fn find<P : AsRef<Path>>(root: P, regex: &Regex) -> Result<Vec<String>, Box<dyn std::error::Error>> {}
  pub fn walk_tree(
      dir : &Path,
      regex: &Regex,
      matches: &mut Vec<String>,
  ) -> Result<(), Box<dyn std::error::Error>> {}
  ```

  

## 2. 实现`-v`参数输出所有遍历文件



- 输入格式为：

  ```shell
  ./target/debug/MyFind -v(可选)
  ```

- 定义变量`flag0`表示是否输入了`-v`参数

  ```rust
  let mut flag0 = 0;
  if args[1] == "-v" {
      flag0 = 1;
  }
  ```

- 在`find`和`walk_tree`函数中增加`flag`参数，在`walk_tree`函数内部遍历文件时增加输出文件路径的语句

  ```rust
  if path.is_dir() {
      walk_tree(&path, regex, matches, flag)?;
  } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
      if flag == 1 {
          println!("遍历：{}", path.to_string_lossy().to_string());
      }
      if regex.is_match(filename) {
          matches.push(path.to_string_lossy().to_string());
      }
  }
  ```

  



## 3. 支持搜索多个`path`并匹配多个正则表达式



- 输入格式为：

  ```shell
  ./target/debug/MyFind -v(可选) <目录数量> <目录1> ... <表达式数量> <表达式1> ...
  ```

- 定义`path_num`和`string_num`变量，类型为`usize`，与`flag`变量结合，分别表示目录的数量和表达式的数量。

  ```rust
  let mut path_num : usize = 0;
  let mut string_num : usize = 0;
  path_num = (&args[flag + 1]).parse::<usize>().unwrap();
  string_num = (&args[flag + 2 + path_num]).parse::<usize>().unwrap();
  ```

- 初始化表示单次搜索目录和表达式的`pattern`和`file_path`变量，使用双重循环，遍历所有目录和表达式参数。

  ```rust
  let mut pattern = &args[flag + 2 + path_num];
  let mut file_path = &args[flag + 1];
  for i in 1..=path_num {
      for j in 1..=string_num {
          pattern = &args[flag + 2 + path_num + j];
          file_path = &args[flag + 1 + i];
          ...
      }
  }
  ```



## 4. 实现命令行彩色输出



- 添加`colored`库，在需要变色的参数后调用对应的颜色函数，如：

  ```rust
  println!("{},内容如下：", file.yellow());
  ```



## 5. 额外实现：增加`-display`参数按行输出文件内容



- 输入格式为：`./target/debug/MyFind -v(可选) -display(可选) <目录数量> <目录1> ... <表达式数量> <表达式1> ...`，`-v`位于第二个或第三个参数的位置。

- 通过`if`语句判断`args[1]`和`args[2]`是否为`"-display"`，使用变量`flag1`来表示是否需要显示文件信息。

  ```rust
  let mut flag1 = 0;
  if args[1] == "-display" || args[2] == "-display" {
      flag1 = 1;
  }
  ```

- 在输出结果的部分，使用`File`和`BufReader`库，通过`flag`的值判断进行按行读取并输出文件内容。

  ```rust
  if flag1 == 1 {
      println!("{},内容如下：", file.yellow());
      let file_content = File::open(file).unwrap();
      let reader = BufReader::new(file_content);
      for line in reader.lines() {
          let line = line.unwrap();
          println!("{}", line);
      }
  }
  else if flag1 == 0 {
      println!("{}", file.yellow());
  }
  ```





## 5. 测试结果



### 测试文件



![image-20230904215402607](/images/image-20230904215402607.png)

![image-20230904215425177](/images/image-20230904215425177.png)

![image-20230904215444145](/images/image-20230904215444145.png)



### 单次匹配测试



```shell
./target/debug/MyFind 1 ~/rust_test1 1 test
```

![image-20230904215751467](/images/image-20230904215751467.png)



### 多次匹配测试



```shell
./target/debug/MyFind 1 ~/rust_test1 2 test others
```

![image-20230904215938780](/images/image-20230904215938780.png)

```shell
./target/debug/MyFind 2 ~/rust_test1 ~/rust_test2 2 test others
```

![image-20230904220029773](/images/image-20230904220029773.png)



### `-v`参数测试



```shell
./target/debug/MyFind -v 2 ~/rust_test1 ~/rust_test2 1 test 
```

![image-20230904224635418](/images/image-20230904224635418.png)



### `-display`参数测试



```shell
./target/debug/MyFind -v -display 1 ~/rust_test1 2 test others
```

![image-20230904224801339](/images/image-20230904224801339.png)





### 目录不存在报错测试



```shell
./target/debug/MyFind 1 ~/empty 1 test 
```

![image-20230904220424333](/images/image-20230904220424333.png)







