use aurelia::database::entity::recipe::Recipe;

use crate::util::{create_jwt, get_tc};

#[tokio::test]
async fn test_get_recipes() {
    let mut c = get_tc().await;
    let jwt = create_jwt(&c.configuration).await;
    c.set_jwt(jwt);

    let res = c.get_recipes().await.unwrap();
    assert_eq!(200, res.status());

    let recipes: Vec<Recipe> = res.json().await.unwrap();

    assert_eq!(1, recipes.len());
    assert_eq!("tim@elbart.com", recipes.get(0).unwrap().user.email);
    assert_eq!(None, recipes.get(0).unwrap().link);
}
