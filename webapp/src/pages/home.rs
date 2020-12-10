use mogwai::prelude::*;
use crate::Route;

pub fn build_root() -> ViewBuilder<HtmlElement> {
    builder!(
        <section class="frow">
            <div class="frow col-md-1-2 credentials-dark full-height hidden-xs hidden-sm">
                           
            </div>
            <div class="frow col-md-1-2 p-0 direction-column full-height credentials-dark">
                <div class="frow">
                    <svg height="500" viewBox="0 0 132.29166 132.29167" width="500" xmlns="http://www.w3.org/2000/svg"><path d="m0 11.24999h285.75v206.07613c-24.87528 23.26408-32.82298 22.34794-80.32007 22.94279-58.89907.73765-205.42993-60.12161-205.42993-113.66739z" fill="#fff" stroke-linecap="round" stroke-width=".972781" transform="matrix(.46296296 0 0 .57762839 0 -6.498314)"/></svg>
                </div>
                <div class="frow direction-column row-between p-20">
                    <div class="frow direction-column width-100 column-start">
                        <h2 class="font-primary color-primary">"Study"</h2>
                        <h2 class="font-primary color-primary">"Anywhere"</h2>
                        <h2 class="font-primary color-primary">"Anytime"</h2>
                    </div>
                    <div class="frow direction-column mt-20">
                        <div class="frow">
                            <a href=String::from(Route::Home) class="width-100 text-center font-primary link-button">"Student
                                Portal"</a>
                        </div>
                        <div class="frow mt-10">
                            <a href=String::from(Route::Settings) class="font-primary clear-button mr-20">"Staff Portal"</a>
                            <a href=String::from(Route::Settings) class="font-primary clear-button">"Admin Portal"</a>
                        </div>
                    </div>
                    <div class="frow direction-column">
                        <p class="text-white text-center font-80">"By continuing you agree to our Terms of Service & Privacy Policy"</p>
                    </div>
                </div>
            </div>
        </section>
    )
}