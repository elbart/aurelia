INSERT INTO "user" ("email", "password")
VALUES (
        'tim@elbart.com',
        '$2a$12$7dPYVtEZ7WLLKfem5Pcy3OpjkpzqlpeadawENLyci4UEj25Qwj3TG'
    );
INSERT INTO "tag" ("creator_id", "name")
SELECT id,
    'vegetarian'
FROM "user"
WHERE email = 'tim@elbart.com'
UNION ALL
SELECT id,
    'vegan'
FROM "user"
WHERE email = 'tim@elbart.com'
UNION ALL
SELECT id,
    'gluten-free'
FROM "user"
WHERE email = 'tim@elbart.com';
INSERT INTO "ingredient" ("name")
VALUES ('tomato'),
    ('lentil'),
    ('pepper');
INSERT INTO "recipe" ("name", "user_id")
SELECT 'nice lentil bowl',
    id
FROM "user"
WHERE email = 'tim@elbart.com';
INSERT INTO "recipe_ingredient" ("recipe_id", "ingredient_id")
SELECT recipe.id,
    ingredient.id
FROM recipe,
    ingredient
WHERE recipe.name = 'nice lentil bowl'
    AND ingredient.name = 'tomato'
UNION ALL
SELECT recipe.id,
    ingredient.id
FROM recipe,
    ingredient
WHERE recipe.name = 'nice lentil bowl'
    AND ingredient.name = 'lentil'
UNION ALL
SELECT recipe.id,
    ingredient.id
FROM recipe,
    ingredient
WHERE recipe.name = 'nice lentil bowl'
    AND ingredient.name = 'pepper';
INSERT INTO "ingredient_tag" ("ingredient_id", "tag_id")
SELECT ingredient.id,
    tag.id
FROM ingredient,
    tag
WHERE ingredient.name = 'pepper'
    AND tag.name = 'gluten-free'
UNION ALL
SELECT ingredient.id,
    tag.id
FROM ingredient,
    tag
WHERE ingredient.name = 'pepper'
    AND tag.name = 'vegan'
UNION ALL
SELECT ingredient.id,
    tag.id
FROM ingredient,
    tag
WHERE ingredient.name = 'pepper'
    AND tag.name = 'vegetarian'
UNION ALL
SELECT ingredient.id,
    tag.id
FROM ingredient,
    tag
WHERE ingredient.name = 'lentil'
    AND tag.name = 'gluten-free'
UNION ALL
SELECT ingredient.id,
    tag.id
FROM ingredient,
    tag
WHERE ingredient.name = 'lentil'
    AND tag.name = 'vegetarian'
UNION ALL
SELECT ingredient.id,
    tag.id
FROM ingredient,
    tag
WHERE ingredient.name = 'lentil'
    AND tag.name = 'vegan'
UNION ALL
SELECT ingredient.id,
    tag.id
FROM ingredient,
    tag
WHERE ingredient.name = 'tomato'
    AND tag.name = 'vegetarian'
UNION ALL
SELECT ingredient.id,
    tag.id
FROM ingredient,
    tag
WHERE ingredient.name = 'tomato'
    AND tag.name = 'gluten-free'
UNION ALL
SELECT ingredient.id,
    tag.id
FROM ingredient,
    tag
WHERE ingredient.name = 'tomato'
    AND tag.name = 'vegan';