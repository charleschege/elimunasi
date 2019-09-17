use serde_derive::{Serialize, Deserialize};


    /// The types of users for the project
    /// ## Example
    /// ```
    /// use::en_client::User;
    /// let foo = User::Admin;
    /// ```
#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq)]
pub enum User {
        /// Admin User
        /// ## Examples
        /// ```
        /// use en_client::User;
        /// let user_role = User::Admin;
        /// ```
    Admin,
        /// SubAdmin User
        /// ## Examples
        /// ```
        /// use en_client::User;
        /// let user_role = User::SubAdmin;
        /// ```
    SubAdmin,
        /// Accounts User
        /// ## Examples
        /// ```
        /// use en_client::User;
        /// let user_role = User::Accounts;
        /// ```
    Accounts,
        /// Lecturer User
        /// ## Examples
        /// ```
        /// use en_client::User;
        /// let user_role = User::Lecturer;
        /// ```
    Lecturer,
        /// Student User
        /// ## Examples
        /// ```
        /// use en_client::User;
        /// let user_role = User::Student;
        /// ```
    Student,
        /// Unspecified
        /// ## Examples
        /// ```
        /// use en_client::User;
        /// let user_role = User::Unspecified;
        /// ```
    Unspecified,
}

impl Default for User {
    fn default() -> Self {
        Self::Unspecified
    }
}
    /// Convert `User` to borrowed string
    /// ## Example
    /// ```
    /// use en_client::{User, stringify_user};
    /// stringify_user(User::Admin);
    /// ```
pub fn stringify_user(user: User) -> &'static str {
    match user {
        User::Admin => "Admin",
        User::SubAdmin => "SubAdmin",
        User::Accounts => "Accounts",
        User::Lecturer => "Lecturer",
        User::Student => "Student",
        _ => "",
    }
}
    /// Convert `User` from a borrowed string to and enum `User`
    /// ## Example
    /// ```
    /// use en_client::{User, enumify_user};
    /// enumify_user("Admin");
    /// ```
pub fn enumify_user(value: &str) -> User {
    match value {
        "Admin" => User::Admin,
        "SubAdmin" => User::SubAdmin,
        "Accounts" => User::Accounts,
        "Lecturer" => User::Lecturer,
        "Student" => User::Student,
        _ => User::Unspecified,
    }
}