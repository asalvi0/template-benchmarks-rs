use criterion;
use minijinja::{context, Environment};
use serde_derive::Serialize;

pub fn big_table(b: &mut criterion::Bencher<'_>, size: &usize) {
    let mut table = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(i);
        }
        table.push(inner);
    }

    let mut env = Environment::new();
    env.add_template(
        "big-table.html",
        include_str!("../templates/big-table.html"),
    )
    .unwrap();

    let tmpl = env.get_template("big-table.html").unwrap();
    b.iter(|| tmpl.render(context! {names => ["John", "Peter"]}));
}

#[derive(Serialize)]
struct Team {
    name: String,
    score: u8,
}

pub fn teams(b: &mut criterion::Bencher<'_>, _: &usize) {
    let ctx = context! {
        year => 2015,
        teams => vec![
            Team {
                name: "Jiangsu".into(),
                score: 43,
            },
            Team {
                name: "Beijing".into(),
                score: 27,
            },
            Team {
                name: "Guangzhou".into(),
                score: 22,
            },
            Team {
                name: "Shandong".into(),
                score: 12,
            },
        ]
    };

    let mut env = Environment::new();
    env.add_template("teams.html", include_str!("../templates/teams.html"))
        .unwrap();

    let tmpl = env.get_template("teams.html").unwrap();
    b.iter(|| tmpl.render(ctx.clone()));
}
