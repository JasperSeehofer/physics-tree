use leptos::prelude::*;

use crate::components::auth::register_form::RegisterForm;

/// Register page — centered form card on a full-screen void background.
/// Route: /register
#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-void flex items-center justify-center px-4">
            <div class="max-w-sm w-full bg-bark-dark rounded-card p-8">
                // Logo
                <div class="flex items-center gap-2 justify-center mb-6">
                    <h1 class="text-xl font-bold text-petal-white">"PhysicsTree"</h1>
                </div>
                <h2 class="text-xl font-bold text-petal-white text-center mb-1">
                    "Create your account"
                </h2>
                <p class="text-sm font-normal text-mist text-center mb-6">
                    "Start tracking your physics journey"
                </p>
                <RegisterForm />
                <p class="text-sm text-mist text-center mt-4">
                    "Already have an account? "
                    <a href="/login" class="text-sky-teal hover:underline">"Log in"</a>
                </p>
            </div>
        </div>
    }
}
