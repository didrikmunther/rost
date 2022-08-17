use phf::phf_map;

// Mappings to nasm system calls
pub static SYSTEM_CALLS: phf::Map<&'static str, usize> = phf_map! {
    "print" => 1,
};