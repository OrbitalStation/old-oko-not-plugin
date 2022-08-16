fn main() -> oko::error::Result <()> {
    println!("{:#?}", oko::parse::parse("/home/rtarasenko/oko-project/src/core.oko")?);

    oko::error::Result(Ok(()))
}
