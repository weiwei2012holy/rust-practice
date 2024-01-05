## RUST 初体验

### 入门基础

> rust 学习有点难度!!! 别太高估自己看一遍文档就入门[🐶]

#### 教程/文档

- Rust 语言圣经 https://course.rs/about-book.html
- Rust 练习实践 https://zh.practice.rs/why-exercise.html
- 通过例子学 Rust  https://rustwiki.org/zh-CN/rust-by-example/
- rust cargo 语言官方 https://doc.rust-lang.org/cargo/index.html
- rustdoc 文档生成 https://doc.rust-lang.org/rustdoc/index.html

#### 环境及配置

- 安装 RUST [略过...]
- 选择编辑器
    - VS code 安装 RUST 扩展
    - goland 安装 RUST 扩展
    - [x] RustRover 预览版 [本项目使用]
- 使用 `cargo` 包管理
- 使用 `rustdoc` 生成文档

#### Cargo

```shell
# 创建项目
cargo new project

# 在一个目录初始化项目
cargo init

# 运行项目
cargo run

# build 成运行文件 
cargo build # 默认是 debug 模式
cargo build --release # 

# build 前检查编译是否通过,速度快
cargo check
# 更新依赖
cargo update
```

#### rustdoc 文档生成

```shell
# 生成文档
rustdoc src/main.rs

# 使用 cargo,其实是一样的功能,只是带了默认的参数
cargo doc

```

##### 如何编写?

使用 `///` 或者 `#[doc]`

```text
/// This is a doc comment.
#[doc = r" This is a doc comment."]

```

#### 项目结构

- 项目 Package :
    - 包含有独立的 Cargo.toml 文件，以及因为功能性被组织在一起的一个或多个包
    - `cargo new my-package`
- 库 library :
    - 只能包含一个库(library)类型的包，但是可以包含多个二进制可执行类型的包
    - `cargo new my-lib --lib`
    - 如何将 lib 的代码分离到其他文件
        1. 在 lib 定义 mod,在同级目录下添加同名文件,在文件内部完成代码编写
        2. 分离到目录:
            1. 在 lib 定义 mod,在同级目录下创建同名目录,同时创建同名文件,在文件中定义 文件夹内mod
            2. 在 lib 定义 mod,在同级目录下创建同名目录,在目录下创建 `mod.rs` 文件,在其中定义 mod,在文件夹内添加其他文件代码


- 包 crate
- 模块 Module

```text
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs

```

- `Cargo.toml` and `Cargo.lock` 根目录.
- `src` 源码目录.
- The default library file is src/lib.rs.
- The default executable file is src/main.rs.
- Other executables can be placed in src/bin/.
- Benchmarks go in the `benches` directory.
- Examples go in the `examples` directory.
- `tests` 集成测试目录.

##### Cargo.toml 和 Cargo.lock

Cargo.toml 和 Cargo.lock 是 cargo 的核心文件，它的所有活动均基于此二者。

- Cargo.toml 是 cargo 特有的项目数据描述文件。它存储了项目的所有元配置信息，如果 Rust 开发者希望 Rust
  项目能够按照期望的方式进行构建、测试和运行，那么，必须按照合理的方式构建 Cargo.toml。

- Cargo.lock 文件是 cargo 工具根据同一项目的 toml 文件生成的项目依赖详细清单，因此我们一般不用修改它，只需要对着
  Cargo.toml
  文件撸就行了。

### 测试驱动

运行 cargo test 会从以下位置搜索

- `src` 源码内定义的用例
- 项目 `tests` 集成测试文件夹

#### 一个最简单的单元测试

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

```

其中，`tests`就是一个测试模块，`it_works`则是我们的主角：测试函数。

#### 条件编译 #[cfg(test)]，

上面代码中的`#[cfg(test)]`标注可以告诉 Rust 只有在`cargo test`时才编译和运行模块`tests`
，其它时候当这段代码是空气即可，例如在`cargo build`时。这么做有几个好处：

- 节省构建代码时的编译时间
- 减小编译出的可执行文件的体积

#### 集成测试

一个标准的 Rust 项目，在它的根目录下会有一个`tests`目录，该目录就是用来存放集成测试的，Cargo
会自动来此目录下寻找集成测试文件。我们可以在该目录下创建任何文件，Cargo 会对每个文件都进行自动编译

#### Cargo 运行测试

**运行所有测试**

```bash
cargo test
```

**测试是输出print**

```bash
cargo test -- --show-output
```

**运行单个测试**
> 参数后接测试的名称即可

```bash
cargo test one_hundred
```

**运行多个测试**

- cargo test [方法名称部分字符]，类似于`PHP`中定义的`group`，运行所有包含这个字符的用例
- cargo test [模块名称]

```bash
# 运行所有包含it字符的测试方法
cargo test it 
# 运行tests整个测试模块
cargo test tests 
```

#### 注解

**#[test]**
> 标记一个方法为测试用例，测试模块中也可以定义非测试模块，这里类似于PHP中测试用例都是用test作为前缀

**#[ignore]**
> 标记一个测试用例在运行的时候跳过，类似于PHP中的`markTestSkipped`
> 如果想要运行这些用例，可以参考`cargo test tests -- --ignored`
>
>

### 语法基础

#### 变量&变量绑定

```shell
cargo test variables
```
- 使用 `let` 定义变量,可以指定类型也可以自动推导;默认的变量是不可变的;定义可变变量使用 `let mut`
- 使用`"`双引号定义字符串,单引号定义 `char`
- 可以使用 `_` 让数字变量更具可读性
- 使用`{}` 格式化字符串,自动推断类型
- 使用下划线`_`开头忽略未使用的变量
- 支持变量解构
- `const`定义常量
  - 常量不可变,那么和 let 定义的变量不可变有什么区别呢? 常量是编译时就确定了,变量不可变只是在初始化后不能改变
- 变量遮蔽(shadowing): 
  - 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明;
  - 对`mut`声明的变量无效,直接赋值即可更改

#### 变量类型
- 数值类型: 有符号整数 (i8, i16, i32, i64, isize)、 无符号整数 (u8, u16, u32, u64, usize) 、浮点数 (f32, f64)、以及有理数、复数
- 字符串：字符串字面量和字符串切片 &str
- 布尔类型： true和false
- 字符类型: 表示单个 Unicode 字符，存储为 4 个字节
- 单元类型: 即 () ，其唯一的值也是 ()
