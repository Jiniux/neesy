macro_rules! not_a_function_err {
    ($fn_name:ident) => {
        Err(format!("{} is not a function.", $fn_name))
    }
}

macro_rules! cannot_find_var_err {
    ($var_name:ident) => {
        Err(format!("Cannot find variable {}.", $var_name))
    }
}

macro_rules! cannot_assign_void_to_var_err {
    ($var_name:ident) => {
        Err(format!("Cannot assign void to {}.", $var_name))
    }
}

macro_rules! cannot_assign_to_builtin_err {
    ($var_name:ident) => {
        Err(format!("Can't assign a value to {} builtin function.", $var_name))
    }
}

macro_rules! not_enough_params_err {
    ($name:ident, $param_count:ident, $actual_params:ident) => {
        Err(format!(
            "{} requires {} param(s), {} given",
            $name,
            $param_count,
            $actual_params.len()
        ));    
    }
}

macro_rules! invalid_operands_err {
    ($op_name:tt, $x:ident, $y:ident) => {
        Err(format!("Cannot {} {:?} with {:?}", $op_name, $x, $y))
    };
}