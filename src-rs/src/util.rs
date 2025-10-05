#[macro_export]
macro_rules! wrap_errs {
    ( $e:expr, $m:expr ) => {
        async { Ok($e) }.await.wrap_err_with(|| $m)
    };
}

#[macro_export]
macro_rules! bind_commands {
    [ $($ident:ident),+ ] => {
        {
            // Create or update TypeScript bindings when debugging
            #[cfg(debug_assertions)]
            tauri_specta::ts::export(
                specta::collect_types![$($ident),+],
                "../src/lib/bindings.ts",
            ).expect("Failed to export types");

            // Create the Tauri plugin to register commands
            tauri::generate_handler![$($ident),+]
        }
    };
}

pub type StrResult<T, E = Vec<String>> = core::result::Result<T, E>;

pub trait StringifyResult<T> {
    fn stringify(self) -> core::result::Result<T, Vec<String>>;
}

impl<T> StringifyResult<T> for eyre::Result<T> {
    fn stringify(self) -> core::result::Result<T, Vec<String>> {
        match self {
            Ok(o) => Ok(o),
            Err(e) => Err(e.chain().map(|e| e.to_string()).collect()),
        }
    }
}
