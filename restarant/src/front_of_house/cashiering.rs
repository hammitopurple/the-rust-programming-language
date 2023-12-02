#[allow(dead_code)]
fn collect_money() {
    // Notice that we can use serving here, even though in front_of_house,
    // serving is private. That's because cashiering is a sibling to serving
    super::serving::take_payment();

    // However, because take_order() is not pub, we can't use it here
    // super::serving::take_order(); // error: function `take_order` is private
}
