#[macro_export]
macro_rules! bail_nom_error {
    ($e:expr) => {
        return Err(nom::Err::Error($e));
    };
}
