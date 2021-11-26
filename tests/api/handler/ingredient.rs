use aurelia::database::entity::ingredient::Ingredient;

use crate::util::{create_jwt, get_tc};

#[tokio::test]
async fn test_get_ingredients() {
    let mut c = get_tc().await;
    let jwt = create_jwt(&c.configuration).await;
    c.set_jwt(jwt);

    let res = c.get_ingredients().await.unwrap();
    assert_eq!(200, res.status());

    let ingredients: Vec<Ingredient> = res.json().await.unwrap();

    assert_eq!(ingredients.len(), 3);
}
