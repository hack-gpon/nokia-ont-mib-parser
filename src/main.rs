use std::str;

const FILE_HEAD_LEN: usize = 0x8;
const ME_STRUCT_LEN: usize = 0x24;
const ME_ENTRY_STRUCT_LEN: usize = 0x1C;
const MAX_ME_ENTRIES: usize = 0x34;

#[derive(Debug)]
struct MeEntry {
    name: String,
    num_attrs: u8,
    num_entries: u8,
    data_len: usize,
    data_post: Vec<u8>,
    attributes: Vec<MeAttribute>
}

#[derive(Debug)]
struct MeAttribute {
    name: String,
    data_start: usize,
    data_len: usize,
    unk: usize,
}

fn main() {
    let data = read_file();
    println!("{} len", data.len());
    let head = &data[0..FILE_HEAD_LEN];
    println!("{:02X?}", head);
    let mut last = FILE_HEAD_LEN;
    let mut me_table: Vec<MeEntry> = Vec::new();
    let mut me_entry_count = 0;
    while me_entry_count < MAX_ME_ENTRIES {
        me_entry_count = me_entry_count + 1;
        let ont_g = &data[last..last + ME_STRUCT_LEN];
        last = last + ME_STRUCT_LEN;
        let mut me_entry = to_me_head(ont_g);
        let me_entry_end = last + (me_entry.num_attrs as usize) * ME_ENTRY_STRUCT_LEN;
        while last < me_entry_end {
            let first_entry = &data[last..last + ME_ENTRY_STRUCT_LEN];
            let me_attr = to_me_entry(&first_entry);
            me_entry.attributes.push(me_attr);
            //println!("\t{:?}", me_attr);
            last = last + ME_ENTRY_STRUCT_LEN;
        }
        println!("[0x{:02}] {:?}", me_entry_count, me_entry);
        me_table.push(me_entry);
    }
    for entry in &me_table {
        for entry_id in 0..entry.num_entries {
            println!("Reading entry {} of {}", entry_id + 1, entry.name);
            for attribute in &entry.attributes {
                let vec_data = &data[last + attribute.data_start..last + attribute.data_start + attribute.data_len];
                let str_data = String::from_utf8_lossy(&vec_data);
                println!("{}[{}]\t{}='{}'\t{:02X?}", entry.name, entry_id, attribute.name, str_data, vec_data)
            }
            last = last + entry.data_len
        }
    }
}

fn to_me_entry(data: &[u8]) -> MeAttribute {
    MeAttribute {
        name: str::from_utf8(&data[0..25]).unwrap().replace("\0", "").to_owned(),
        data_len: data[25] as usize,
        data_start: data[27] as usize,
        unk: data[26] as usize,
    }
}

fn to_me_head(data: &[u8]) -> MeEntry {
    return MeEntry {
        name: str::from_utf8(&data[0..27]).unwrap().replace("\0", "").to_owned(),
        num_attrs: data[27],
        num_entries: data[29],
        data_len: data[31] as usize,
        data_post: data[28..].to_owned(),
        attributes: Vec::new()
    };
}

fn read_file() -> Vec<u8> {
    match std::fs::read("ont.mib") {
        Ok(bytes) => { return bytes; }
        Err(e) => {
            panic!("{}", e);
        }
    }
}