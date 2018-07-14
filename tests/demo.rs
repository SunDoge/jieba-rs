extern crate jiebars;

#[test]
fn demo() {
    let seg_list = jiebars::cut("我来到北京清华大学", true, true);
    assert_eq!(
        "我/ 来到/ 北京/ 清华/ 清华大学/ 华大/ 大学",
        &seg_list.join("/ ")
    );

    let seg_list = jiebars::cut("我来到北京清华大学", false, true);
    assert_eq!("我/ 来到/ 北京/ 清华大学", &seg_list.join("/ "));

    let seg_list = jiebars::cut("他来到了网易杭研大厦", false, true);
    assert_eq!(
        "他, 来到, 了, 网易, 杭研, 大厦",
        &seg_list.join(", ")
    );

    let seg_list = jiebars::cut_for_search(
        "小明硕士毕业于中国科学院计算所，后在日本京都大学深造",
        true
    );
    assert_eq!(
        "小明, 硕士, 毕业, 于, 中国, 科学, 学院, 科学院, 中国科学院, 计算, 计算所, ，, 后, 在, 日本, 京都, 大学, 日本京都大学, 深造",
        &seg_list.join(", ")
    );

    assert_eq!(
        "如果/放到/post/中将/出错/。",
        jiebars::cut("如果放到post中将出错。", false, false).join("/")
    );

    
}
