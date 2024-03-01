use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    method: String,
    arg1: Option<String>,
    arg2: Option<String>
}

pub fn start() -> (String, Option<String>, Option<String>) {

    let args = Cli::parse();

    (args.method, args.arg1, args.arg2)

}

// pub fn read() -> String {
//
//     let mut said= String::new();
//
//     match stdin().read_line(&mut said) {
//         Ok(value) => {
//             said
//         }
//         Err(error) => panic!("{error}")
//     }
//
// }