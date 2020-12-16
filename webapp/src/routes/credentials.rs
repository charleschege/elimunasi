use crate::pages;
use log::trace;
use mogwai::prelude::*;

/// Here we enumerate all our app's routes.
#[derive(Clone, Debug, PartialEq)]
pub enum Credentials {
    RootPage,
    AdminLogin,
    StudentLogin,
    StaffLogin,
    ResetPassword,
}

/// We'll use TryFrom::try_from to convert the window's url hash into a Credentials.
impl TryFrom<&str> for Credentials {
    type Error = String;

    fn try_from(s: &str) -> Result<Credentials, String> {
        trace!("route try_from: {}", s);
        // remove the scheme, if it has one
        let hash_split = s.split("#").collect::<Vec<_>>();
        let after_hash = match hash_split.as_slice() {
            [_, after] => Ok(after),
            _ => Err(format!("route must have a hash: {}", s)),
        }?;

        let paths: Vec<&str> = after_hash.split("/").collect::<Vec<_>>();
        trace!("route paths: {:?}", paths);

        

        let curr = paths.as_slice()[1];
        trace!("path: {:?}", curr);
        trace!("HERE");

        match paths.as_slice()[1] {
            "" => Ok(Credentials::RootPage),
            "student_login" => Ok(Credentials::StudentLogin),
            "admin_login" => Ok(Credentials::AdminLogin),
            "staff_login" => Ok(Credentials::StaffLogin),
            "reset_password" => Ok(Credentials::ResetPassword),
            r => Err(format!("unsupported route: {:?}", r)),
        }
    }
}

#[cfg(test)]
mod test_route_try_from {
    use super::*;

    #[test]
    fn can_convert_string_to_route() {
        let s = "https://localhost:8080/#/";
        assert_eq!(Credentials::try_from(s), Ok(Credentials::RootPage));
    }
}

/// Convert the route into its hashed string.
/// This should match the inverse conversion in TryFrom above.
impl From<Credentials> for String {
    fn from(route: Credentials) -> String {
        match route {
            Credentials::RootPage => "#/".into(),
            Credentials::StudentLogin => "#/student_login".into(),
            Credentials::AdminLogin => "#/admin_login".into(),
            Credentials::StaffLogin => "#/staff_login".into(),
            Credentials::ResetPassword => "#/reset_password".into(),
        }
    }
}

/// We can convert a route into a ViewBuilder in order to embed it in a gizmo.
/// This is just a suggestion for this specific example. The general idea is
/// to use the route to inform your app that it needs to change the page. This
/// is just one of many ways to accomplish that.
impl From<&Credentials> for ViewBuilder<HtmlElement> {
    fn from(route: &Credentials) -> Self {
        match route {
            Credentials::RootPage => pages::root(),
            Credentials::StudentLogin => pages::student_login(),
            Credentials::AdminLogin => pages::admin_login(),
            Credentials::StaffLogin => pages::staff_login(),
            Credentials::ResetPassword => pages::reset_password(),
        }
    }
}

impl From<&Credentials> for View<HtmlElement> {
    fn from(route: &Credentials) -> Self {
        ViewBuilder::from(route).into()
    }
}

/// Here we'll define some helpers for displaying information about the current route.
impl Credentials {
    pub fn nav_root_class(&self) -> String {
        match self {
            Credentials::RootPage => "nav-link active",
            _ => "nav-link",
        }
        .to_string()
    }

    pub fn nav_student_class(&self) -> String {
        match self {
            Credentials::StudentLogin => "nav-link active",
            _ => "nav-link",
        }
        .to_string()
    }

    pub fn nav_admin_class(&self) -> String {
        match self {
            Credentials::AdminLogin => "nav-link active",
            _ => "nav-link",
        }
        .to_string()
    }

    pub fn nav_staff_class(&self) -> String {
        match self {
            Credentials::StaffLogin => "nav-link active",
            _ => "nav-link",
        }
        .to_string()
    }
}
