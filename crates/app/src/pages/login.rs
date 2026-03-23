use leptos::prelude::*;

use crate::components::auth::login_form::LoginForm;

/// Login page — centered form card on a full-screen void background.
/// Route: /login
#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-void flex items-center justify-center px-4">
            <div class="max-w-sm w-full bg-bark-dark rounded-card p-8">
                // Logo
                <div class="flex items-center gap-2 justify-center mb-6">
                    <h1 class="text-xl font-bold text-petal-white">"PhysicsTree"</h1>
                </div>
                <h2 class="text-xl font-bold text-petal-white text-center mb-1">"Welcome back"</h2>
                <p class="text-sm font-normal text-mist text-center mb-6">
                    "Log in to track your progress"
                </p>
                <LoginForm />
                <p class="text-sm text-mist text-center mt-4">
                    "Don't have an account? "
                    <a href="/register" class="text-sky-teal hover:underline">"Create one"</a>
                </p>
            </div>
        </div>
    }
}
