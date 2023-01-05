#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr,)*) => {
        {
        let mut hm = ::std::collections::HashMap::new();
        $(hm.insert($key, $value);)*
                hm
        }
    };
    ($($key:expr => $value:expr),*) => {
         {
        let mut hm = ::std::collections::HashMap::new();
        $(hm.insert($key, $value);)*
                hm
        }
    };
}



//macros_rules! vec [
//    ($($e:expr), *) => ({
//        let mut _temp = ::std::vec::Vec::new();
//        $(_temp.push($e);)*
//            _temp
//    })
//];

