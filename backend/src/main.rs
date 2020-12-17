use async_executor::Executor;
use futures_lite::future;
use tide::{
    prelude::*,
    Body,
    Request,
};

mod routes;
mod db_model;

use routes::{student_login, admin_login, staff_login};

fn main() -> anyhow::Result<()> {
    let executor = Executor::new();

    let task = executor.spawn(async {
        tide::log::start();
        let mut app = tide::new();

        app.at("/").get(|_| async { Ok(Body::from_file("resources/index.html").await?) });
        app.at("/resources").serve_dir("resources/").unwrap();

        app.at("/admin_login").post(admin_login);
        app.at("/staff_login").post(staff_login);
        app.at("/student_login").post(student_login);

        app.listen("0.0.0.0:5500").await.unwrap();
    });

    future::block_on(executor.run(task));

    Ok(())
}