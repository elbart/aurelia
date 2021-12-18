use aurelia::database::entity::ingredient::Ingredient;

use crate::util::get_tc;

#[tokio::test]
async fn test_get_ingredients() {
    let c = get_tc().await.authenticated().await;

    let res = c.get_ingredients().await.unwrap();
    assert_eq!(200, res.status());

    let ingredients: Vec<Ingredient> = res.json().await.unwrap();

    assert_eq!(ingredients.len(), 3);
}
