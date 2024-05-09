use anyhow::Context;
use rustgit::{read_ext::ReadExt, Repository};
use std::{
    fs::File,
    io::{BufRead, BufReader, ErrorKind, Read},
    path::PathBuf,
    usize,
};

// Read header of index and return the number of entries

fn read_header(reader: &mut impl Read) -> anyhow::Result<usize> {
    let header_signature = reader.read_exact_4()?;
    anyhow::ensure!(
        matches!(&header_signature, b"DIRC"),
        "index signature is not DIRC"
    );

    let version_number = u32::from_be_bytes(reader.read_exact_4()?);
    anyhow::ensure!(version_number == 2, "rustgit only support index version 2");
    let entry_count = u32::from_be_bytes(reader.read_exact_4()?);
    Ok(entry_count as usize)
}

pub fn ls_files() -> anyhow::Result<()> {
    let repository = Repository::search_and_open(&std::env::current_dir()?)?;

    let index_file = File::open(repository.git_directory.join("index"));
    if matches!(&index_file, Err(e) if e.kind() == ErrorKind::NotFound) {
        return Ok(());
    }
    let index_file = index_file.context("open .git/index file")?;
    let mut reader = BufReader::new(index_file);

    let entry_count = read_header(&mut reader)?;

    for _ in 0..entry_count {
        const MIN_ENTRY_SIZE: usize = 62;
        let buf = reader.read_exact_n::<MIN_ENTRY_SIZE>()?;

        let path_length = u16::from_be_bytes(buf[60..].try_into()?);

        let mut path = vec![];
        let length = reader
            .read_until(0, &mut path)
            .context("failed to read header from tree object")?;
        assert_eq!(length, usize::from(path_length + 1));

        let path = PathBuf::from(std::str::from_utf8(&path)?);

        println!("{}", path.to_string_lossy());

        // consume padding bits
        reader.seek_relative(
            i64::try_from(8 - (MIN_ENTRY_SIZE + path_length as usize) % 8 - 1).unwrap(),
        )?;
    }

    Ok(())
}
