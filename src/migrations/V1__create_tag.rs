use barrel::{backend::Pg, types, Migration};

pub fn migration() -> String {
    let mut m = Migration::new();

    m.create_table("tag", |t| {
        t.add_column("id", types::uuid().primary(true));
        t.add_column("name", types::varchar(255).nullable(false).unique(true));
    });

    m.make::<Pg>()
}
