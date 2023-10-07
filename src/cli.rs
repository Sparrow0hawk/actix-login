pub struct Args {
    pub host: String,
    pub port: i32,
}

pub fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut host: String = String::from("127.0.0.1");
    let mut port = 8080;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("host") => {
                host = parser.value()?.parse()?;
            }
            Short('p') | Long("port") => {
                port = parser.value()?.parse()?;
            }
            Long("help") => {
                println!("Usage: actix_login [-h|--host=X.X.X.X] [-p|--port=X.X.X.X]");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args { host, port })
}
