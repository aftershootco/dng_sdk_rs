// struct Guard<const U: bool>;
// trait Protect {}
// impl Protect for Guard<true> {}
// fn f<const N: usize>()
// where
//     Guard<
//         {
//             const fn check<const N: usize>() -> bool {
//                 if !N > 0 {
//                     panic!("guard evaluated to false")
//                 }
//                 true
//             }
//             check::<{ N }>()
//         },
//     >: Protect,
// {
// }
