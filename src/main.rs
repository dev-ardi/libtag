use libtag::{initialize_thread, scan_directory};

fn main() {
    initialize_thread();
    scan_directory("/home/ardi/Sync");
}
