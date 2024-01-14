use std::process::exit;
use crate::constants::{
    CONSIDERING_LATEST_ID_MSG,
    ENTRY_NOT_FOUND_MSG,
    NO_ENTRIES_FOUND_MSG,
    SUCCESSFULLY_DELETED_ENTRY_MSG
};
use crate::entries_repository::{
    delete_by_id,
    get_latest_entry_id
};

pub(crate) fn delete_entry(param: String) {
    let id: String;
    if param.eq("--latest") {
        let latest_entry_id = get_latest_entry_id();
        if latest_entry_id.is_err() {
            println!("{}", NO_ENTRIES_FOUND_MSG);
            exit(1);
        }
        id = latest_entry_id.unwrap();
        println!("{} {}", CONSIDERING_LATEST_ID_MSG, id.clone());
    } else {
        id = param;
    }

    if delete_by_id(id.clone()) > 0 {
        println!("{} {}", SUCCESSFULLY_DELETED_ENTRY_MSG, id);
    } else {
        println!("{}", ENTRY_NOT_FOUND_MSG);
        exit(1);
    }
}


