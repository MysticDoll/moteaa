use regex::Regex;

const MOTEMEN: [[(u8, u8, u8); 4]; 4]  = [
    [( 195, 213, 227 ), ( 252, 255, 246 ), ( 19, 58, 137 ),   ( 157, 167, 185 )],
    [( 129, 82, 38 ),   ( 255, 224, 211 ), ( 248, 199, 182 ), ( 218, 46, 32 )],
    [( 164, 172, 193 ), ( 254, 196, 184 ), ( 255, 179, 162 ), ( 225, 107, 93 )],
    [( 174, 179, 183 ), ( 55, 62, 68 ), ( 83, 32, 13 ),    ( 89, 90, 95 )]
];

fn main() {
    let re = Regex::new(r"(?<size>\d+)x?").unwrap();
    let size_arg= std::env::args()
        .nth(1)
        .unwrap_or("1x".to_owned());
    let size = re
        .captures(&size_arg)
        .map(|caps| caps
             .name("size")
             .map_or("1", |cap| cap.as_str())
            .to_owned()
        )
        .unwrap_or("1".to_owned())
        .parse::<usize>()
        .unwrap_or(1);
        
    let result = MOTEMEN.into_iter().map(|row| {
        vec![
            format!("{}\x1b[m\n", row
                    .into_iter()
                    .map(|color| format!(
                        "{}{}",
                        format!("\x1b[48;2;{};{};{}m", color.0, color.1, color.2),
                        vec!["  ".to_owned(); size].concat()
                    ))
                    .collect::<Vec<String>>()
                    .concat()); size
        ].concat()
    })
        .collect::<Vec<String>>()
        .concat();
    println!("{}\x1b[m", result);
}
