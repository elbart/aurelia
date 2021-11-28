use std::rc::Rc;

use sql_press::{
    change::ChangeSet,
    column::{real, varchar},
    sql_dialect::postgres::Postgres,
};

pub fn migration() -> String {
    let mut cs = ChangeSet::new();

    cs.alter_table("recipe_ingredient", |t| {
        t.add_column(real("quantity").build());
        t.add_column(varchar("unit", Some(255)).build());
    });

    let ddl = cs.get_ddl(Rc::new(Postgres::new()));
    let f = file!();
    tracing::info!("Migration for file: {}\n{}", f, ddl);
    ddl
}
