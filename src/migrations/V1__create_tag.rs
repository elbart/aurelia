use std::rc::Rc;

use sql_press::{
    change::ChangeSet,
    column::{uuid, varchar, DefaultConstraint},
    sql_dialect::postgres::Postgres,
};

pub fn migration() -> String {
    let mut cs = ChangeSet::new();

    cs.run_script("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";");

    cs.create_table("tag", |t| {
        t.add_column(
            uuid("id")
                .primary(true)
                .default(DefaultConstraint::Plain("uuid_generate_v4()".into()))
                .build(),
        );
        t.add_column(varchar("name", None).build());
    });

    let ddl = cs.get_ddl(Rc::new(Postgres::new()));
    let f = file!();
    tracing::info!("Migration for file: {}\n{}", f, ddl);
    ddl
}
