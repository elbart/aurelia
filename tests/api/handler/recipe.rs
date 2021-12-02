use aurelia::database::entity::recipe::Recipe;

use crate::util::get_tc;

#[tokio::test]
async fn test_get_recipes() {
    let c = get_tc().await.authenticated().await;

    let res = c.get_recipes().await.unwrap();
    assert_eq!(200, res.status());

    let recipes: Vec<Recipe> = res.json().await.unwrap();

    assert_eq!(1, recipes.len());
    assert_eq!("tim@elbart.com", recipes.get(0).unwrap().user.email);
    assert_eq!(None, recipes.get(0).unwrap().link);
    assert_eq!(3, recipes.get(0).unwrap().ingredients.len());
    assert_eq!(
        3,
        recipes
            .get(0)
            .unwrap()
            .ingredients
            .get(0)
            .unwrap()
            .ingredient
            .tags
            .len()
    );

    println!("{:?}", recipes);
}
