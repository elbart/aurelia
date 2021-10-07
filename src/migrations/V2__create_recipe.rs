use std::rc::Rc;

use sql_press::{
    change::ChangeSet,
    column::{uuid, varchar},
    sql_dialect::postgres::Postgres,
};

pub fn migration() -> String {
    let mut cs = ChangeSet::new();

    cs.create_table("ingredient", |t| {
        t.add_column(uuid("id").primary(true).build());
        t.add_column(varchar("name", Some(255)).build());
    });

    cs.create_table("recipe", |t| {
        t.add_column(uuid("id").primary(true).build());
        t.add_column(varchar("name", Some(255)).build());
        t.add_column(varchar("link", Some(4096)).build());
        t.add_column(varchar("description", Some(4096)).build());
    });

    cs.create_table("recipe_ingredient", |t| {
        t.add_column(uuid("ingredient_id").build());
        t.add_column(uuid("recipe_id").build());

        t.add_foreign_index("ingredient_id", "ingredient", "id", None);
        t.add_foreign_index("recipe_id", "recipe", "id", None);
    });

    let ddl = cs.get_ddl(Rc::new(Postgres::new()));
    let f = file!();
    tracing::info!("Migration for file: {}\n{}", f, ddl);
    ddl
}
