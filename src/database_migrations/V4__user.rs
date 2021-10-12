use std::rc::Rc;

use sql_press::{
    change::ChangeSet,
    column::{uuid, varchar, DefaultConstraint},
    sql_dialect::postgres::Postgres,
};

pub fn migration() -> String {
    let mut cs = ChangeSet::new();

    cs.create_table("user", |t| {
        t.add_column(
            uuid("id")
                .default(DefaultConstraint::Plain("uuid_generate_v4()".into()))
                .primary(true)
                .build(),
        );
        t.add_column(
            varchar("email", Some(255))
                .not_null(true)
                .unique(true)
                .build(),
        );
        t.add_column(varchar("password", Some(1024)).build());
    });

    cs.alter_table("tag", |t| {
        t.add_column(uuid("creator_id").not_null(true).build());
        t.add_foreign_index("creator_id", "user", "id", None);
    });

    cs.alter_table("recipe", |t| {
        t.add_column(uuid("user_id").not_null(true).build());
        t.add_foreign_index("user_id", "user", "id", None);
    });

    let ddl = cs.get_ddl(Rc::new(Postgres::new()));
    let f = file!();
    tracing::info!("Migration for file: {}\n{}", f, ddl);
    ddl
}
