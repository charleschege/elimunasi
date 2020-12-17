use tide::{
    prelude::*,
    Body,
    Request,
};

pub async fn student_login(mut req: Request<()>) -> tide::Result {
    //let get_post_body: = req.body_json().await?;
    Ok("Student Login Response".into())
}