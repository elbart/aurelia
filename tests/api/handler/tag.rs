use aurelia::database::entity::tag::Tag;

use crate::util::get_tc;

#[tokio::test]
async fn test_get_recipes() {
    let c = get_tc().await.authenticated().await;

    let res = c.get_tags().await.unwrap();
    assert_eq!(200, res.status());

    let tags: Vec<Tag> = res.json().await.unwrap();

    assert_eq!(tags.len(), 3);
}
