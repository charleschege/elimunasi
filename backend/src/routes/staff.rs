use tide::{
    prelude::*,
    Body,
    Request,
};

pub async fn staff_login(mut req: Request<()>) -> tide::Result {
    //let get_post_body: = req.body_json().await?;
    Ok("Staff Login Response".into())
}