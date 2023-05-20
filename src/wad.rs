use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

/**
 * WAD header taken from the first 12 bytes of the WAD file.
 */
struct Header {
    // 4 character identification, either 'IWAD' or 'PWAD'
    identification: Identification,

    // integer specifying the number of lumps (files) in the WAD
    numlumps: i32,

    // integer holding a pointer to the location of the directory.
    infotablesofs: i32,
}

/**
 * The directory associates names of lumps with the data that belong to them.
 * It consists of a number of entries, each with a length of 16 bytes.
 */
struct Directory {
    // An integer holding a pointer to the start of the lump's data in the file
    filepos: i32,

    // An integer representing the size of the lump in bytes
    size: i32,

    // A string defining the lump's name
    name: String
}

/**
 * WAD file type, either IWAD or PWAD.
 */
enum Identification {
    // full game
    IWAD,
    // game mod
    PWAD
}

impl FromStr for Identification {
    type Err = ();

    fn from_str(input: &str) -> Result<Identification, Self::Err> {
        match input {
            "IWAD" => Ok(Identification::IWAD),
            "PWAD" => Ok(Identification::PWAD),
            _      => Err(()),
        }
    }
}

impl Display for Identification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Identification::IWAD => write!(f, "IWAD"),
            Identification::PWAD => write!(f, "PWAD"),
        }
    }
}

pub fn load_wad_file(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    const BUFFER_LEN: usize = 512;
    let mut buffer: [u8; BUFFER_LEN] = [0u8; BUFFER_LEN];
    let mut file: File = File::open(filepath)?;
    let mut wad: Vec<u8> = Vec::new();

    loop {
        let read_count = file.read(&mut buffer)?;
        wad.append(&mut buffer.to_vec());

        if read_count != BUFFER_LEN { break; }
    }

    // Read WAD header
    let signature: Identification = Identification::from_str(std::str::from_utf8(&wad[..=3]).unwrap())
        .expect("Not a valid WAD file!");
    let num_lumps: i32 = i32::from_le_bytes(wad[4..=7].try_into().unwrap());
    let off_fat: i32 = i32::from_le_bytes(wad[8..=11].try_into().unwrap());

    // Read WAD directory
    let mut directory: Vec<Directory> = Vec::new();
    let mut index = off_fat;
    while usize::try_from(index+16).unwrap() < wad.len() {
        directory.push(read_directory_entry(&wad, usize::try_from(index).unwrap()));
        index += 16;
    }

    println!("--- HEADER ---");
    println!("WadIdent: {}", signature);
    println!("NumLumps: {}", num_lumps);
    println!("OffFAT:   {}", off_fat);
    println!("--- DIRECTORY ---");
    for dir in directory {
        println!("filepos: {}, size: {}, name: {}", dir.filepos, dir.size, dir.name);
    }

    Ok(())
}

fn read_directory_entry(wad: &Vec<u8>, index: usize) -> Directory {
    Directory {
        filepos: i32::from_le_bytes(wad[index..index+4].try_into().unwrap()),
        size: i32::from_le_bytes(wad[index+4..index+8].try_into().unwrap()),
        name: String::from_utf8_lossy(&wad[index+8..index+16]).to_string()
    }
}
