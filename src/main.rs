macro_rules! elm {
    () => {};
    (
        $fn_ty:ident: $return_type:ident
        $fn_def:ident =
            $r:expr
    ) => {
        fn $fn_ty() -> $return_type {
            $r
        }
    };
    (
        $fn_ty:ident: $param_type:ident -> $return_type:ident
        $fn_def:ident $param_name:ident =
            $r:expr
    ) => {
        fn $fn_ty($param_name: $param_type) -> $return_type {
            $r
        }
    };
    (
        $fn_ty:ident: $param_type_1:ident -> $param_type_2:ident -> $return_type:ident
        $fn_def:ident $param_name_1:ident $param_name_2:ident =
            $r:expr
    ) => {
        fn $fn_ty($param_name_1: $param_type_1, $param_name_2: $param_type_2) -> $return_type {
            $r
        }
    };
}

fn main() {
    elm!(
        identity: i32 -> i32
        identity a =
            a
    );
    let identity = identity(10);

    println!("{identity}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_compile_simple_function() {
        elm!(
            main: i32
            main =
                42
        );
    }

    #[test]
    fn should_create_simple_function() {
        elm!(
            main: i32
            main =
                42
        );
        let expected = 42;
        let input = main();
        assert_eq!(input, expected);
    }

    #[test]
    fn should_create_one_arg_fn() {
        elm!(
            main: i32 -> i32
            main a =
                42
        );
        let expected = 42;
        let input = main(1);
        assert_eq!(input, expected);
    }

    #[test]
    fn should_create_one_arg_identity() {
        elm!(
            identity: i32 -> i32
            identity a =
                a
        );
        let expected = 42;
        let input = identity(42);
        assert_eq!(input, expected);
    }

    #[test]
    fn should_compile_add_function() {
        elm!(
            add: i32 -> i32 -> i32
            add rhs lhs =
                rhs + lhs
        );
    }

    #[test]
    fn should_create_add_function() {
        elm!(
            add: i32 -> i32 -> i32
            add rhs lhs =
                rhs + lhs
        );
        let expected = 2;
        let input = add(1, 1);
        assert_eq!(input, expected);
    }
}
