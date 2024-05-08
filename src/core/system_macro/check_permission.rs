#[macro_export]
macro_rules! find_min {
    ($x:expr $(, $y:expr)+) => {{
        let mut min_val = $x;
        $(
            if $y < min_val {
                min_val = $y;
            }
        )+
        min_val
    }};
}