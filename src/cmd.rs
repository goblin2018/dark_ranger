use clap::{App, Arg, SubCommand};



fn parse_cmd() -> App<'static, 'static> {

let app = App::new("auto")
  .version("1.0")
  .about("This is an auto generate tool").arg(
    Arg::with_name("input")
  )

}