#[cfg(test)]
mod tests {
    use core::panic;

    use super::get_backtrace;
    #[test]
    fn it_works() {
        println!("{}", get_backtrace());
        panic!();
    }
}
use backtrace;

fn get_backtrace() -> String {
    let bt = backtrace::Backtrace::new();
    format!("{:?}", bt)
}

uniffi_macros::include_scaffolding!("pn");