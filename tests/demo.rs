extern crate jiebars;

#[test] 
fn demo() {
    let seg_list = jiebars::cut("我来到北京清华大学", true, true);
    assert_eq!("我/ 来到/ 北京/ 清华/ 清华大学/ 华大/ 大学", &seg_list.join("/ "));

    let seg_list = jiebars::cut("我来到北京清华大学", false, true);
    assert_eq!("我/ 来到/ 北京/ 清华大学", &seg_list.join("/ "));

    let seg_list = jiebars::cut("他来到了网易杭研大厦", false, true);
    assert_eq!("他, 来到, 了, 网易, 杭研, 大厦", &seg_list.join(", "));
}