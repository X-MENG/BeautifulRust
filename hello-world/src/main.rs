fn main() {
    println!("{}", hello());
}
// 添加'static生命周期标注后
// hello函数返回的字符串的生命周期将贯穿整个程序的运行时间
// 所以不会出现悬垂引用
pub fn hello() -> &'static str {
    "Hello, world!"
}