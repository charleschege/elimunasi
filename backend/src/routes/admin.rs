use tide::{
    prelude::*,
    Body,
    Request,
};

pub async fn admin_login(mut req: Request<()>) -> tide::Result {
    //let get_post_body: = req.body_json().await?;
    Ok("Admin Login Response Body".into())
}