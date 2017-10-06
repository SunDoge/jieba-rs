extern crate jiebars;

#[test] 
fn demo() {
    let seg_list = jiebars::cut("我来到北京清华大学", true, true);
    assert_eq!("我/ 来到/ 北京/ 清华/ 清华大学/ 华大/ 大学", &seg_list.join("/ "));
}