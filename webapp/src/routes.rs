use mogwai::prelude::*;

pub fn home() -> ViewBuilder<HtmlElement> {
    
    builder! {
        <main class="container">
            <div class="overlay">
                "I am home"
            </div>
        </main>
    }
}

pub fn login() -> ViewBuilder<HtmlElement> {
    
    builder! {
        <main class="container">
            <div class="overlay">
                "I am login"
            </div>
        </main>
    }
}
pub fn logout() -> ViewBuilder<HtmlElement> {
    
    builder! {
        <main class="container">
            <div class="overlay">
                "I am logout"
            </div>
        </main>
    }
}

pub fn not_found() -> ViewBuilder<HtmlElement> {
    builder! {
        <h1>"Not Found"</h1>
    }
}