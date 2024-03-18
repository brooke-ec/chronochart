#[macro_export]
macro_rules! wrap_errs {
    ( $e:expr, $m:expr ) => {
        async { Ok($e) }.await.wrap_err_with(|| $m)
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
