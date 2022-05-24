#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub tarnish_core); // synthesized by LALRPOP

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
