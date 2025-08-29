use std::io;

fn main() {
    println!("请输入一个字符串：");
    
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let reverse_string = input
                .trim()
                .chars()
                .rev()
                .collect::<String>();
            println!("反转后的字符串是：{}", reverse_string);
        },
        Err(error) => {
            println!("读取输入时发生错误：{}", error);
        }
    }
}
