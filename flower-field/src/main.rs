fn get_flower_count(garden: &[&str], r: i32, c: i32) ->char {
    match garden[r as usize].as_bytes()[c as usize] {
        b'*' => '*',    // ‘*’代表花，只考虑不是花的格子
        _ => {
            let maxr = garden.len() as i32 - 1;         // 确定花园的row的上界
            let maxc = garden[r as usize].len() as i32 - 1;    // 确定花园的col的上界   
            match ((r - 1).max(0)..=(r + 1).min(maxr))
                // 对每个r, 生成一组rc组合，然后将结果存储成一维数组
                .flat_map(|newr| ((c - 1).max(0)..=(c + 1).min(maxc)).map(move |newc| (newr, newc)))
                // 对一维数组进行过滤，如果该坐标对应的位置是'*'，则执行count操作
                .filter(|&(r, c)| garden[r as usize].as_bytes()[c as usize] == b'*')
                .count()
            {
                0 => ' ',
                count => (count as u8 + b'0') as char,
            }
        }
    }
}

fn mark_flower_count(garden: &[&str]) -> Vec<String> {
    (0..garden.len())
        .map(|r| {
            (0..garden[r].len())
                .map(|c| get_flower_count(garden, r as i32, c as i32))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}

#[test]
fn no_rows() {
    let input = &[];
    let expected: &[&str] = &[];
    let actual = mark_flower_count(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn no_columns() {
    let input = &[""];
    let expected = &[""];
    let actual = mark_flower_count(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn no_flowers() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "   ",
        "   ",
        "   ",
    ], &[
        "   ",
        "   ",
        "   ",
    ]);
    let actual = mark_flower_count(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn garden_full_of_flowers() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "***",
        "***",
        "***",
    ], &[
        "***",
        "***",
        "***",
    ]);
    let actual = mark_flower_count(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn flower_surrounded_by_spaces() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "   ",
        " * ",
        "   ",
    ], &[
        "111",
        "1*1",
        "111",
    ]);
    let actual = mark_flower_count(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn space_surrounded_by_flowers() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "***",
        "* *",
        "***",
    ], &[
        "***",
        "*8*",
        "***",
    ]);
    let actual = mark_flower_count(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn horizontal_line() {
    let input = &[" * * "];
    let expected = &["1*2*1"];
    let actual = mark_flower_count(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn horizontal_line_flowers_at_edges() {
    let input = &["*   *"];
    let expected = &["*1 1*"];
    let actual = mark_flower_count(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn vertical_line() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        " ",
        "*",
        " ",
        "*",
        " ",
    ], &[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
    let actual = mark_flower_count(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn vertical_line_flowers_at_edges() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "*",
        " ",
        " ",
        " ",
        "*",
    ], &[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
    let actual = mark_flower_count(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn cross() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "  *  ",
        "  *  ",
        "*****",
        "  *  ",
        "  *  ",
    ], &[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
    let actual = mark_flower_count(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn large_garden() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        " *  * ",
        "  *   ",
        "    * ",
        "   * *",
        " *  * ",
        "      ",
    ], &[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
    let actual = mark_flower_count(input);
    assert_eq!(actual, expected);
}

fn main() {
    println!("Hello, world!");
}
