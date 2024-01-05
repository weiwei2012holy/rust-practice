//如果 要将lib 里面的内容拆分出去，可以使用如下的方式
// 1. 在 lib 中定义模块
// 2. 在同名的文件中完成模块实现
pub mod random;
pub mod utils;

pub mod tools;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //分离 lib 到当前目录文件
        random::float::random();
        random::int::random();
        random::string::random();
        //分离到文件夹 1
        utils::hash::md5();
        utils::hash::crc32();

        //分离到文件夹 2
        tools::lock::lock();
        tools::lock::unlock();
    }
}
