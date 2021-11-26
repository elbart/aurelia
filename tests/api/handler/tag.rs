use aurelia::database::entity::tag::Tag;

use crate::util::{create_jwt, get_tc};

#[tokio::test]
async fn test_get_recipes() {
    let mut c = get_tc().await;
    let jwt = create_jwt(&c.configuration).await;
    c.set_jwt(jwt);

    let res = c.get_tags().await.unwrap();
    assert_eq!(200, res.status());

    let tags: Vec<Tag> = res.json().await.unwrap();

    assert_eq!(tags.len(), 3);
}
