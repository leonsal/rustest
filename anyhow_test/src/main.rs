use anyhow::Context;


fn main() -> anyhow::Result<()> {

    let v = get_value_from_file()?;
    println!("value:{}", v);
    Ok(())
}


fn get_value_from_file() -> anyhow::Result<i32> {

    // Reads contents of file into string
    let text = std::fs::read_to_string("invalid_file.xxx").context("error reading file to parse")?;

    // Parses read string into an i32 value
    let val: i32 = text.parse().context("error parsing text from file")?;

    // Returns parsed value
    Ok(val)
}

