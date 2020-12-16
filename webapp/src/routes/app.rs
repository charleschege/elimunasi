use mogwai::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Route4App {
    Dashboard,
    Study,
    Courses,
    Profile {
        username: String,
        is_favorites: bool,
    },
}

/// Here we enumerate all our app's routes.
#[derive(Clone, Debug, PartialEq)]
pub enum Route4Credentials {
    RootPage,
    Settings,
    Profile {
        username: String,
        is_favorites: bool,
    },
}

/// We'll use TryFrom::try_from to convert the window's url hash into a Route.
impl TryFrom<&str> for Route {
    type Error = String;

    fn try_from(s: &str) -> Result<Route, String> {
        trace!("route try_from: {}", s);
        // remove the scheme, if it has one
        let hash_split = s.split("#").collect::<Vec<_>>();
        let after_hash = match hash_split.as_slice() {
            [_, after] => Ok(after),
            _ => Err(format!("route must have a hash: {}", s)),
        }?;

        let paths: Vec<&str> = after_hash.split("/").collect::<Vec<_>>();
        trace!("route paths: {:?}", paths);

        match paths.as_slice() {
            [""] => Ok(Route::Home),
            ["", ""] => Ok(Route::Home),
            ["", "settings"] => Ok(Route::Settings),
            // Here you can see that profile may match two different routes -
            // '#/profile/{username}' and '#/profile/{username}/favorites'
            ["", "profile", username] => Ok(Route::Profile {
                username: username.to_string(),
                is_favorites: false,
            }),
            ["", "profile", username, "favorites"] => Ok(Route::Profile {
                username: username.to_string(),
                is_favorites: true,
            }),
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
        assert_eq!(Route::try_from(s), Ok(Route::Home));
    }
}

/// Convert the route into its hashed string.
/// This should match the inverse conversion in TryFrom above.
impl From<Route> for String {
    fn from(route: Route) -> String {
        match route {
            Route::Home => "#/".into(),
            Route::Settings => "#/settings".into(),
            Route::Profile {
                username,
                is_favorites,
            } => {
                if is_favorites {
                    format!("#/profile/{}/favorites", username)
                } else {
                    format!("#/profile/{}", username)
                }
            }
        }
    }
}

/// We can convert a route into a ViewBuilder in order to embed it in a gizmo.
/// This is just a suggestion for this specific example. The general idea is
/// to use the route to inform your app that it needs to change the page. This
/// is just one of many ways to accomplish that.
impl From<&Route> for ViewBuilder<HtmlElement> {
    fn from(route: &Route) -> Self {
        match route {
            Route::Home => pages::build_root(),
            Route::Settings => builder! {
                <main>
                    <h1>"Update your settings"</h1>
                </main>
            },
            Route::Profile {
                username,
                is_favorites,
            } => builder! {
                <main>
                    <h1>{username}"'s Profile"</h1>
                    {if *is_favorites {
                        Some(builder!{
                            <h2>"Favorites"</h2>
                        })
                    } else {
                        None
                    }}
                </main>
            },
        }
    }
}

impl From<&Route> for View<HtmlElement> {
    fn from(route: &Route) -> Self {
        ViewBuilder::from(route).into()
    }
}

/// Here we'll define some helpers for displaying information about the current route.
impl Route {
    pub fn nav_home_class(&self) -> String {
        match self {
            Route::Home => "nav-link active",
            _ => "nav-link",
        }
        .to_string()
    }

    pub fn nav_settings_class(&self) -> String {
        match self {
            Route::Settings { .. } => "nav-link active",
            _ => "nav-link",
        }
        .to_string()
    }

    pub fn nav_profile_class(&self) -> String {
        match self {
            Route::Profile { .. } => "nav-link active",
            _ => "nav-link",
        }
        .to_string()
    }
}
