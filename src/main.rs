fn main() -> oko::error::Result <()> {
    println!("{:#?}", oko::parse::parse("src/core.oko")?);

    oko::error::Result(Ok(()))
}
