fn main() {
    // `config.ini.example` được embed thẳng vào binary qua `include_str!`
    // (xem `utils::app_config`). File `config/config.ini` thực tế sẽ được tạo/migrate
    // lúc runtime nên build.rs không cần sinh nó nữa.
    tauri_build::build()
}
