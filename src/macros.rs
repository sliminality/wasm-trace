#[macro_use]
#[allow(unused_macros)]
macro_rules! map {
    ( $( $k:expr => $v:expr ),+ ) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($k, $v.to_owned()); )+
            map
    }};
}
