#![allow(dead_code)]
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    table: HashMap<String, String>,
    last_address: i32,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = HashMap::new();
        table.insert("SP".to_string(), "000000000000000".to_string());
        table.insert("LCL".to_string(), "000000000000001".to_string());
        table.insert("ARG".to_string(), "000000000000010".to_string());
        table.insert("THIS".to_string(), "000000000000011".to_string());
        table.insert("THAT".to_string(), "000000000000100".to_string());
        table.insert("R0".to_string(), "000000000000000".to_string());
        table.insert("R1".to_string(), "000000000000001".to_string());
        table.insert("R2".to_string(), "000000000000010".to_string());
        table.insert("R3".to_string(), "000000000000011".to_string());
        table.insert("R4".to_string(), "000000000000100".to_string());
        table.insert("R5".to_string(), "000000000000101".to_string());
        table.insert("R6".to_string(), "000000000000110".to_string());
        table.insert("R7".to_string(), "000000000000111".to_string());
        table.insert("R8".to_string(), "000000000001000".to_string());
        table.insert("R9".to_string(), "000000000001001".to_string());
        table.insert("R10".to_string(), "000000000001010".to_string());
        table.insert("R11".to_string(), "000000000001011".to_string());
        table.insert("R12".to_string(), "000000000001100".to_string());
        table.insert("R13".to_string(), "000000000001101".to_string());
        table.insert("R14".to_string(), "000000000001110".to_string());
        table.insert("R15".to_string(), "000000000001111".to_string());
        table.insert("SCREEN".to_string(), "100000000000000".to_string());
        table.insert("KBD".to_string(), "110000000000000".to_string());

        Self {
            table,
            last_address: 15,
        }
    }

    pub fn add_entry(&mut self, symbol: &str, address: &str) {
        self.table.insert(symbol.to_string(), address.to_string());
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> Option<&str> {
        self.table.get(symbol).map(|s| -> &str { s })
    }

    pub fn insert_variable_symbol(&mut self, symbol: &str) {
        let new_addr = &format!("{:015b}", self.last_address + 1);
        self.add_entry(symbol, new_addr);
        self.last_address += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_symbol_table1() {
        let mut st = SymbolTable::new();
        st.add_entry("key", "value");

        assert_eq!(st.contains("key"), true);
        assert_eq!(st.contains("key2"), false);
        assert_eq!(st.get_address("key"), Some("value"));
        assert_eq!(st.get_address("key2"), None);

        assert_eq!(st.get_address("R9"), Some("000000000001001"));
        assert_eq!(st.get_address("SCREEN"), Some("100000000000000"));
        assert_eq!(st.get_address("KBD"), Some("110000000000000"));
    }
}
