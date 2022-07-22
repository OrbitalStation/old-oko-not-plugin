fn main() -> oko::error::Result <()> {
    println!("{:#?}", oko::parse::parse("code.txt")?);

    oko::error::Result(Ok(()))
}
