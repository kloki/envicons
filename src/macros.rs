#[macro_export]
macro_rules! contains {
    ($str:expr) => {
        $crate::Rule::Contains($str.to_string())
    };
}

#[macro_export]
macro_rules! equals {
    ($str:expr) => {
        $crate::Rule::Equals($str.to_string())
    };
}

#[macro_export]

macro_rules! never {
    () => {
        $crate::Rule::Never
    };
}

#[macro_export]
macro_rules! envicon {
    ($icon:expr, $container_rule:expr, $envar_rule:expr) => {
        $crate::Envicon::new($icon.to_string(), $container_rule, $envar_rule)
    };
}
