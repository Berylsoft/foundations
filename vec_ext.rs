#[macro_export]
macro_rules! vec_ext_append {
    {$vec:expr, [$(
        $(@if ($if_cond:expr) $if_body:block)?
        $(@for ($for_pat:pat in $for_val:expr) $for_body:block)?
        $(@if let ($if_let_pat:pat = $if_let_val:expr) $if_let_body:block)?
        $(@append($append:expr))?
        $(@extend($extend:expr))?
        $($val:expr)?
    ),*]} => {{$(
        $(if $if_cond {
            $vec.push($if_body);
        })?
        $(for $for_pat in $for_val {
            $vec.push($for_body);
        })?
        $(if let $if_let_pat = $if_let_val {
            $vec.push($if_let_body);
        })?
        $($vec.append($append);)?
        $($vec.extend($extend);)?
        $($vec.push($val);)?
    )*}};
}

#[macro_export]
macro_rules! vec_ext {
    [$($inner:tt)+] => {{
        let mut v = Vec::new();
        $crate::vec_ext_append! {
            v,
            [$($inner)+]
        }
        v
    }};
}
