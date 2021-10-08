use std::rc::Rc;

use sql_press::{
    change::ChangeSet,
    column::{uuid, DefaultConstraint},
    sql_dialect::postgres::Postgres,
};

pub fn migration() -> String {
    let mut cs = ChangeSet::new();

    cs.create_table("ingredient_tag", |t| {
        t.add_column(
            uuid("ingredient_id")
                .default(DefaultConstraint::Plain("uuid_generate_v4()".into()))
                .build(),
        );
        t.add_column(
            uuid("tag_id")
                .default(DefaultConstraint::Plain("uuid_generate_v4()".into()))
                .build(),
        );

        t.add_primary_index(vec!["ingredient_id", "tag_id"]);
        t.add_foreign_index("ingredient_id", "ingredient", "id", None);
        t.add_foreign_index("tag_id", "tag", "id", None);
    });

    let ddl = cs.get_ddl(Rc::new(Postgres::new()));
    let f = file!();
    tracing::info!("Migration for file: {}\n{}", f, ddl);
    ddl
}
