fn main() {
    slint_build::compile("src/components/main_window.slint").unwrap();
    
    println!("cargo:rerun-if-changed=src/components/main_window.slint");
    println!("cargo:rerun-if-changed=src/components/prompt_editor.slint");
    println!("cargo:rerun-if-changed=src/components/history_panel.slint");
}
