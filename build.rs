use sass_rs::Options;

fn main() {
    println!("cargo:rerun-if-changed=assets/main.scss");
    // println!("cargo:rerun-if-changed=tailwind.css");
    // dioxus_tailwindcss::build::npx_tailwindcss("./", "tailwind.css", "assets/tailwind.css").unwrap();
    let res = sass_rs::compile_file("assets/main.scss", Options::default()).unwrap();
    std::fs::write("assets/main.css", res).unwrap();
}
