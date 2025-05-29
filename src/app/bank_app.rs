use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos_router::hooks::{use_navigate, use_location};
use thaw::*;
use crate::app::models::*;
use crate::api::{auth::*, accounts::*, transactions::*, users::{create_user, get_users}};
use wasm_bindgen::JsCast;

/// Root shell component that provides the basic HTML structure
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

/// Main App component that sets up routing and global providers
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    // Global authentication state
    let (auth_token, set_auth_token) = signal(Option::<String>::None);
    let (current_user, set_current_user) = signal(Option::<User>::None);
    
    provide_context(auth_token);
    provide_context(set_auth_token);
    provide_context(current_user);
    provide_context(set_current_user);

    view! {
        <ConfigProvider>
            <ToasterProvider>
                <Stylesheet id="leptos" href="/pkg/bank-app.css" />
                <Title text="SecureBank - Modern Banking Solution" />
                <Router>
                    <main>
                        <Routes fallback=|| "Page not found.".into_view()>
                            <Route path=StaticSegment("") view=LoginPage />
                            <Route path=StaticSegment("/signup") view=SignupPage />
                            <Route path=StaticSegment("/dashboard") view=DashboardPage />
                            <Route path=StaticSegment("/accounts") view=AccountsPage />
                            <Route path=StaticSegment("/transactions") view=TransactionsPage />
                            <Route path=StaticSegment("/transfer") view=TransferPage />
                            <Route path=StaticSegment("/deposit") view=DepositPage />
                            <Route path=StaticSegment("/admin") view=AdminPage />
                        </Routes>
                    </main>
                </Router>
            </ToasterProvider>
        </ConfigProvider>
    }
}

/// Login page component with real authentication
#[component]
fn LoginPage() -> impl IntoView {
    let (email, set_email) = signal(String::from("test@securebank.test"));
    let (password, set_password) = signal(String::from("password123"));
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);
    
    let navigate = use_navigate();
    let set_auth_token = expect_context::<WriteSignal<Option<String>>>();
    let set_current_user = expect_context::<WriteSignal<Option<User>>>();

    let login_action = Action::new(move |_: &()| {
        let email = email.get();
        let password = password.get();
        let navigate = navigate.clone();
        async move {
            set_loading.set(true);
            set_error.set(None);
            
            let request = LoginRequest { email, password };
            
            match login_user(request).await {
                Ok(token) => {
                    set_auth_token.set(Some(token.clone()));
                    
                    // Get current user info
                    match get_current_user(token).await {
                        Ok(Some(user)) => {
                            set_current_user.set(Some(user));
                            let _ = navigate("/dashboard", Default::default());
                        }
                        Ok(None) => {
                            set_error.set(Some("User not found".to_string()));
                        }
                        Err(e) => {
                            set_error.set(Some(format!("Failed to get user info: {}", e)));
                        }
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!("Login failed: {}", e)));
                }
            }
            set_loading.set(false);
        }
    });

    let on_login = move |_| {
        login_action.dispatch(());
    };

    view! {
        <div style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); display: flex; align-items: center; justify-content: center; padding: 1rem; position: relative; overflow: hidden;">
            // Smaller, subtle background elements
            <div style="position: absolute; inset: 0; overflow: hidden; pointer-events: none;">
                <div style="position: absolute; top: -50px; right: -50px; width: 120px; height: 120px; background: rgba(255, 255, 255, 0.1); border-radius: 50%; filter: blur(20px);" class="animate-blob"></div>
                <div style="position: absolute; bottom: -30px; left: -30px; width: 100px; height: 100px; background: rgba(255, 255, 255, 0.08); border-radius: 50%; filter: blur(25px);" class="animate-blob animation-delay-2000"></div>
                <div style="position: absolute; top: 50%; left: 20px; width: 80px; height: 80px; background: rgba(255, 255, 255, 0.06); border-radius: 50%; filter: blur(30px);" class="animate-blob animation-delay-4000"></div>
                    </div>
            
            <div style="position: relative; z-index: 10; width: 100%; max-width: 400px;">
                <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 20px; box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25); overflow: hidden;">
                    <div style="background: linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%); color: white; padding: 2rem; text-align: center;">
                        <div style="display: inline-flex; align-items: center; justify-content: center; width: 60px; height: 60px; background: rgba(255, 255, 255, 0.2); border-radius: 50%; margin-bottom: 1rem;">
                            <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
                            </svg>
                        </div>
                        <h1 style="font-size: 2rem; font-weight: 700; margin-bottom: 0.5rem; letter-spacing: -0.025em;">"SecureBank"</h1>
                        <p style="color: rgba(255, 255, 255, 0.9); font-size: 1rem;">"Welcome back! Please sign in to your account."</p>
                    </div>
                    
                    <div style="padding: 2rem; display: flex; flex-direction: column; gap: 1.5rem;">
                    {move || error.get().map(|err| view! {
                            <div style="background: #fef2f2; border-left: 4px solid #ef4444; color: #dc2626; padding: 1rem; border-radius: 0.5rem;" class="animate-shake">
                                <div style="display: flex; align-items: center;">
                                    <svg style="width: 20px; height: 20px; margin-right: 0.5rem;" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"></path>
                                    </svg>
                                    <span style="font-weight: 500;">{err}</span>
                                </div>
                        </div>
                    })}
                    
                        <div style="display: flex; flex-direction: column; gap: 1.25rem;">
                    <div>
                                <label style="display: block; font-size: 0.875rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem;">"Email Address"</label>
                                <div style="position: relative;">
                                    <div style="position: absolute; inset-y: 0; left: 0; padding-left: 0.75rem; display: flex; align-items: center; pointer-events: none; z-index: 1;">
                                    </div>
                                    <div style="position: relative;">
                        <Input
                                            class="custom-input"
                                            placeholder="Enter your email address"
                            value=(email, set_email)
                        />
                    </div>
                                </div>
                            </div>
                            
                    <div>
                                <label style="display: block; font-size: 0.875rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem;">"Password"</label>
                                <div style="position: relative;">
                                    <div style="position: absolute; inset-y: 0; left: 0; padding-left: 0.75rem; display: flex; align-items: center; pointer-events: none; z-index: 1;">
                                    </div>
                                    <div style="position: relative;">
                        <Input
                                            class="custom-input"
                            input_type=InputType::Password
                            placeholder="Enter your password"
                            value=(password, set_password)
                        />
                    </div>
                                </div>
                            </div>
                        </div>
                        
                        <div style="width: 100%; background: linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%); border-radius: 0.75rem; box-shadow: 0 10px 15px -3px rgba(79, 70, 229, 0.3);">
                    <Button
                                class="custom-button"
                        loading=loading
                        on_click=on_login
                    >
                                {move || if loading.get() {
                                    view! {
                                        <span style="display: flex; align-items: center; justify-content: center;">
                                            "Signing In..."
                                        </span>
                                    }.into_any()
                                } else {
                                    view! {
                                        <span style="display: flex; align-items: center; justify-content: center;">
                                            <svg style="width: 20px; height: 20px; margin-right: 0.5rem;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1"></path>
                                            </svg>
                        "Sign In"
                                        </span>
                                    }.into_any()
                                }}
                    </Button>
                        </div>
                        
                        <div style="background: linear-gradient(135deg, #dbeafe 0%, #e0e7ff 100%); border: 1px solid #bfdbfe; color: #1e40af; padding: 1rem; border-radius: 0.75rem;">
                            <div style="display: flex; align-items: flex-start;">
                                <svg style="width: 20px; height: 20px; color: #3b82f6; margin-top: 0.125rem; margin-right: 0.75rem; flex-shrink: 0;" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path>
                                </svg>
                                <div style="font-size: 0.875rem;">
                                    <p style="font-weight: 600; margin-bottom: 0.25rem;">"Demo Credentials:"</p>
                                    <p style="color: #1d4ed8; margin: 0;">"Email: test@securebank.test"</p>
                                    <p style="color: #1d4ed8; margin: 0;">"Password: password123"</p>
                                </div>
                            </div>
                    </div>
                    
                        <div style="text-align: center; position: relative;">
                            <div style="position: absolute; inset: 0; display: flex; align-items: center;">
                                <div style="width: 100%; border-top: 1px solid #e5e7eb;"></div>
                    </div>
                            <div style="position: relative; display: flex; justify-content: center; font-size: 0.875rem;">
                                <span style="padding: 0 1rem; background: rgba(255, 255, 255, 0.95); color: #6b7280;">"New to SecureBank?"</span>
                </div>
                        </div>
                        
                        <div style="text-align: center;">
                            <a href="/signup" style="display: inline-flex; align-items: center; justify-content: center; width: 100%; max-width: 100%; padding: 0.75rem 1.5rem; border: 2px solid #e5e7eb; border-radius: 0.75rem; color: #374151; background: white; text-decoration: none; transition: all 0.2s; font-weight: 500; font-size: 1rem; box-sizing: border-box; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
                                <svg style="width: 20px; height: 20px; margin-right: 0.5rem; flex-shrink: 0;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"></path>
                                </svg>
                                "Create New Account"
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Signup page component with user registration
#[component]
fn SignupPage() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());
    let (first_name, set_first_name) = signal(String::new());
    let (last_name, set_last_name) = signal(String::new());
    let (phone, set_phone) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);
    let (success, set_success) = signal(false);
    
    let navigate = use_navigate();

    let signup_action = Action::new(move |_: &()| {
        let email = email.get();
        let password = password.get();
        let confirm_password = confirm_password.get();
        let first_name = first_name.get();
        let last_name = last_name.get();
        let phone = phone.get();
        let navigate = navigate.clone();
        
        async move {
            set_loading.set(true);
            set_error.set(None);
            
            // Validation
            if email.is_empty() || password.is_empty() || first_name.is_empty() || last_name.is_empty() {
                set_error.set(Some("Please fill in all required fields".to_string()));
                set_loading.set(false);
                return;
            }
            
            if password != confirm_password {
                set_error.set(Some("Passwords do not match".to_string()));
                set_loading.set(false);
                return;
            }
            
            if password.len() < 6 {
                set_error.set(Some("Password must be at least 6 characters long".to_string()));
                set_loading.set(false);
                return;
            }
            
            let request = RegisterRequest {
                email,
                password,
                first_name,
                last_name,
                phone: if phone.is_empty() { None } else { Some(phone) },
            };
            
            match create_user(request).await {
                Ok(_) => {
                    set_success.set(true);
                    // Redirect to login after 2 seconds
                    let navigate = navigate.clone();
                    spawn_local(async move {
                        gloo_timers::future::TimeoutFuture::new(2000).await;
                        let _ = navigate("/", Default::default());
                    });
                }
                Err(e) => {
                    set_error.set(Some(format!("Registration failed: {}", e)));
                }
            }
            set_loading.set(false);
        }
    });

    let on_signup = move |_| {
        signup_action.dispatch(());
    };

    view! {
        <div style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); display: flex; align-items: center; justify-content: center; padding: 1rem; position: relative; overflow: hidden;">
            // Smaller background elements for signup
            <div style="position: absolute; inset: 0; overflow: hidden; pointer-events: none;">
                <div style="position: absolute; top: -50px; right: -50px; width: 120px; height: 120px; background: rgba(255, 255, 255, 0.1); border-radius: 50%; filter: blur(20px);" class="animate-blob"></div>
                <div style="position: absolute; bottom: -30px; left: -30px; width: 100px; height: 100px; background: rgba(255, 255, 255, 0.08); border-radius: 50%; filter: blur(25px);" class="animate-blob animation-delay-2000"></div>
                <div style="position: absolute; top: 50%; left: 20px; width: 80px; height: 80px; background: rgba(255, 255, 255, 0.06); border-radius: 50%; filter: blur(30px);" class="animate-blob animation-delay-4000"></div>
                    </div>
            
            <div style="position: relative; z-index: 10; width: 100%; max-width: 500px;">
                <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 20px; box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25); overflow: hidden;">
                    <div style="background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%); color: white; padding: 2rem; text-align: center;">
                        <div style="display: inline-flex; align-items: center; justify-content: center; width: 60px; height: 60px; background: rgba(255, 255, 255, 0.2); border-radius: 50%; margin-bottom: 1rem;">
                            <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"></path>
                            </svg>
                        </div>
                        <h1 style="font-size: 2rem; font-weight: 700; margin-bottom: 0.5rem; letter-spacing: -0.025em;">"SecureBank"</h1>
                        <p style="color: rgba(255, 255, 255, 0.9); font-size: 1rem;">"Create your account to get started."</p>
                    </div>
                    
                    <div style="padding: 2rem; display: flex; flex-direction: column; gap: 1.5rem;">
                    {move || {
                        if success.get() {
                            view! {
                                    <div style="text-align: center; padding: 2rem 0;">
                                        <div style="display: inline-flex; align-items: center; justify-content: center; width: 80px; height: 80px; background: #dcfce7; border-radius: 50%; margin-bottom: 1rem;">
                                            <svg style="width: 40px; height: 40px; color: #16a34a;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                            </svg>
                                        </div>
                                        <h3 style="font-size: 1.5rem; font-weight: 600; color: #111827; margin-bottom: 0.5rem;">"Account Created Successfully!"</h3>
                                        <p style="color: #6b7280; margin-bottom: 1rem; font-size: 1rem;">"Welcome to SecureBank! Redirecting you to sign in..."</p>
                                        <div style="display: flex; align-items: center; justify-content: center;">
                                            <svg style="width: 24px; height: 24px; color: #7c3aed;" class="animate-spin" fill="none" viewBox="0 0 24 24">
                                                <circle style="opacity: 0.25;" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                                <path style="opacity: 0.75;" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                            </svg>
                                        </div>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div>
                                    {move || error.get().map(|err| view! {
                                            <div style="background: #fef2f2; border-left: 4px solid #ef4444; color: #dc2626; padding: 1rem; border-radius: 0.5rem; margin-bottom: 1.5rem;" class="animate-shake">
                                                <div style="display: flex; align-items: center;">
                                                    <svg style="width: 20px; height: 20px; margin-right: 0.5rem;" fill="currentColor" viewBox="0 0 20 20">
                                                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"></path>
                                                    </svg>
                                                    <span style="font-weight: 500;">{err}</span>
                                                </div>
                                        </div>
                                    })}
                                    
                                        <div style="display: flex; flex-direction: column; gap: 1.25rem;">
                                            // Personal Information
                                            <div style="background: linear-gradient(135deg, #f3f4f6 0%, #e5e7eb 100%); padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #d1d5db;">
                                                <h3 style="font-size: 1rem; font-weight: 600; color: #374151; margin-bottom: 1rem; display: flex; align-items: center;">
                                                    <svg style="width: 20px; height: 20px; margin-right: 0.5rem; color: #7c3aed;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                                                    </svg>
                                                    "Personal Information"
                                                </h3>
                                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
                                        <div>
                                                        <label style="display: block; font-size: 0.875rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem;">"First Name *"</label>
                                            <Input
                                                            class="custom-input"
                                                placeholder="Enter your first name"
                                                value=(first_name, set_first_name)
                                            />
                                        </div>
                                        <div>
                                                        <label style="display: block; font-size: 0.875rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem;">"Last Name *"</label>
                                            <Input
                                                            class="custom-input"
                                                placeholder="Enter your last name"
                                                value=(last_name, set_last_name)
                                            />
                                                    </div>
                                        </div>
                                    </div>
                                    
                                            // Contact Information
                                            <div style="background: linear-gradient(135deg, #eff6ff 0%, #dbeafe 100%); padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #bfdbfe;">
                                                <h3 style="font-size: 1rem; font-weight: 600; color: #374151; margin-bottom: 1rem; display: flex; align-items: center;">
                                                    <svg style="width: 20px; height: 20px; margin-right: 0.5rem; color: #3b82f6;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 4.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
                                                    </svg>
                                                    "Contact Information"
                                                </h3>
                                                <div style="display: flex; flex-direction: column; gap: 1rem;">
                                    <div>
                                                        <label style="display: block; font-size: 0.875rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem;">"Email Address *"</label>
                                                        <div style="position: relative;">
                                                            <div style="position: absolute; inset-y: 0; left: 0; padding-left: 0.75rem; display: flex; align-items: center; pointer-events: none; z-index: 1;">
                                                            </div>
                                                            <div style="position: relative;">
                                        <Input
                                                                    class="custom-input"
                                            input_type=InputType::Email
                                                                    placeholder="Enter your email address"
                                            value=(email, set_email)
                                        />
                                                            </div>
                                                        </div>
                                    </div>
                                    
                                    <div>
                                                        <label style="display: block; font-size: 0.875rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem;">"Phone Number"</label>
                                                        <div style="position: relative;">
                                                            <div style="position: absolute; inset-y: 0; left: 0; padding-left: 0.75rem; display: flex; align-items: center; pointer-events: none; z-index: 1;">
                                                            </div>
                                                            <div style="position: relative;">
                                        <Input
                                                                    class="custom-input"
                                            input_type=InputType::Tel
                                            placeholder="Enter your phone number"
                                            value=(phone, set_phone)
                                        />
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                    </div>
                                    
                                            // Security Section
                                            <div style="background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%); padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #f59e0b;">
                                                <h3 style="font-size: 1rem; font-weight: 600; color: #374151; margin-bottom: 1rem; display: flex; align-items: center;">
                                                    <svg style="width: 20px; height: 20px; margin-right: 0.5rem; color: #f59e0b;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
                                                    </svg>
                                                    "Security"
                                                </h3>
                                                <div style="display: flex; flex-direction: column; gap: 1rem;">
                                    <div>
                                                        <label style="display: block; font-size: 0.875rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem;">"Password *"</label>
                                                        <div style="position: relative;">
                                                            <div style="position: absolute; inset-y: 0; left: 0; padding-left: 0.75rem; display: flex; align-items: center; pointer-events: none; z-index: 1;">
                                                                
                                                            </div>
                                                            <div style="position: relative;">
                                        <Input
                                                                    class="custom-input"
                                            input_type=InputType::Password
                                                                    placeholder="Create a secure password"
                                            value=(password, set_password)
                                        />
                                                            </div>
                                                        </div>
                                    </div>
                                    
                                    <div>
                                                        <label style="display: block; font-size: 0.875rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem;">"Confirm Password *"</label>
                                                        <div style="position: relative;">
                                                            <div style="position: absolute; inset-y: 0; left: 0; padding-left: 0.75rem; display: flex; align-items: center; pointer-events: none; z-index: 1;">
                                                               
                                                            </div>
                                                            <div style="position: relative;">
                                        <Input
                                                                    class="custom-input"
                                            input_type=InputType::Password
                                            placeholder="Confirm your password"
                                            value=(confirm_password, set_confirm_password)
                                        />
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                    </div>
                                    
                                        <div style="width: 100%; background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%); border-radius: 0.75rem; box-shadow: 0 10px 15px -3px rgba(124, 58, 237, 0.3);">
                                    <Button
                                                class="custom-button"
                                        loading=loading
                                        on_click=on_signup
                                    >
                                                {move || if loading.get() {
                                                    view! {
                                                        <span style="display: flex; align-items: center; justify-content: center;">
                                                            "Creating Account..."
                                                        </span>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <span style="display: flex; align-items: center; justify-content: center;">
                                                            <svg style="width: 20px; height: 20px; margin-right: 0.5rem;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"></path>
                                                            </svg>
                                        "Create Account"
                                                        </span>
                                                    }.into_any()
                                                }}
                                    </Button>
                                        </div>
                                        
                                        <div style="text-align: center; position: relative;">
                                            <div style="position: absolute; inset: 0; display: flex; align-items: center;">
                                                <div style="width: 100%; border-top: 1px solid #e5e7eb;"></div>
                                            </div>
                                        </div>
                                        
                                        <div style="text-align: center;">
                                            <a href="/" style="display: inline-flex; align-items: center; justify-content: center; width: 100%; max-width: 100%; padding: 0.75rem 1.5rem; border: 2px solid #e5e7eb; border-radius: 0.75rem; color: #374151; background: white; text-decoration: none; transition: all 0.2s; font-weight: 500; font-size: 1rem; box-sizing: border-box; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
                                                <svg style="width: 20px; height: 20px; margin-right: 0.5rem; flex-shrink: 0;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1"></path>
                                                </svg>
                                                "Sign In Instead"
                                            </a>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
                </div>
            </div>
        </div>
    }
}

/// Dashboard page component with real data
#[component]
fn DashboardPage() -> impl IntoView {
    let current_user = expect_context::<ReadSignal<Option<User>>>();
    let auth_token = expect_context::<ReadSignal<Option<String>>>();
    let navigate = use_navigate();
    
    // Redirect if not authenticated
    Effect::new(move |_| {
        if auth_token.get().is_none() {
            let _ = navigate("/", Default::default());
        }
    });

    view! {
        <div style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); position: relative; overflow: hidden;">
            // Background elements
            <div style="position: absolute; inset: 0; overflow: hidden; pointer-events: none;">
                <div style="position: absolute; top: -50px; right: -50px; width: 120px; height: 120px; background: rgba(255, 255, 255, 0.1); border-radius: 50%; filter: blur(20px);" class="animate-blob"></div>
                <div style="position: absolute; bottom: -30px; left: -30px; width: 100px; height: 100px; background: rgba(255, 255, 255, 0.08); border-radius: 50%; filter: blur(25px);" class="animate-blob animation-delay-2000"></div>
                <div style="position: absolute; top: 50%; right: 20px; width: 80px; height: 80px; background: rgba(255, 255, 255, 0.06); border-radius: 50%; filter: blur(30px);" class="animate-blob animation-delay-4000"></div>
            </div>
            
            <div style="display: flex; min-height: 100vh; position: relative; z-index: 10;">
                // Sidebar
                <div style="width: 280px; background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border-right: 1px solid rgba(255, 255, 255, 0.2); box-shadow: 5px 0 25px rgba(0, 0, 0, 0.1);">
                <NavigationSidebar />
                </div>
                
                // Main content
                <div style="flex: 1; display: flex; flex-direction: column;">
                    // Header
                    <div style="background: rgba(255, 255, 255, 0.9); backdrop-filter: blur(10px); border-bottom: 1px solid rgba(255, 255, 255, 0.2); padding: 1.5rem 2rem; box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);">
                        <div style="display: flex; justify-content: space-between; align-items: center;">
                            <h1 style="font-size: 2rem; font-weight: 700; color: #1e293b; margin: 0; letter-spacing: -0.025em;">"Dashboard"</h1>
                            <div style="display: flex; align-items: center; gap: 1rem;">
                            {move || current_user.get().map(|user| view! {
                                    <span style="color: #64748b; font-weight: 500;">"Welcome, " {user.first_name} " " {user.last_name}</span>
                            })}
                            <LogoutButton />
                        </div>
                    </div>
                    </div>
                    
                    // Content area
                    <div style="flex: 1; padding: 2rem; background: transparent;">
                    <DashboardContent />
                </div>
                </div>
            </div>
        </div>
    }
}

/// Logout button component
#[component]
fn LogoutButton() -> impl IntoView {
    let set_auth_token = expect_context::<WriteSignal<Option<String>>>();
    let set_current_user = expect_context::<WriteSignal<Option<User>>>();
    let navigate = use_navigate();
    
    let on_logout = move |_| {
        set_auth_token.set(None);
        set_current_user.set(None);
        let _ = navigate("/", Default::default());
    };

    view! {
        <Button on_click=on_logout>"Logout"</Button>
    }
}

/// Navigation sidebar component
#[component]
fn NavigationSidebar() -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();

    // Helper function to determine if a path is active
    let is_active = move |path: &str| {
        let current_path = location.pathname.get();
        current_path == path || (path == "/dashboard" && current_path == "/")
    };

    // Active button styles
    let active_style = "display: flex; align-items: center; width: 100%; padding: 0.875rem 1rem; border: none; background: linear-gradient(135deg, rgba(124, 58, 237, 0.1) 0%, rgba(79, 70, 229, 0.1) 100%); color: #7c3aed; border-radius: 0.75rem; font-weight: 600; font-size: 0.875rem; cursor: pointer; transition: all 0.2s; text-align: left; border-left: 3px solid #7c3aed;";
    
    // Inactive button styles
    let inactive_style = "display: flex; align-items: center; width: 100%; padding: 0.875rem 1rem; border: none; background: rgba(255, 255, 255, 0.5); color: #64748b; border-radius: 0.75rem; font-weight: 500; font-size: 0.875rem; cursor: pointer; transition: all 0.2s; text-align: left; border-left: 3px solid transparent;";

    view! {
        <div style="padding: 2rem 1.5rem; height: 100vh; display: flex; flex-direction: column;">
            <div style="margin-bottom: 3rem; text-align: center;">
                <div style="display: inline-flex; align-items: center; justify-content: center; width: 60px; height: 60px; background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%); border-radius: 1rem; margin-bottom: 1rem;">
                    <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
                    </svg>
            </div>
                <h2 style="font-size: 1.5rem; font-weight: 700; color: #1e293b; margin: 0; letter-spacing: -0.025em;">"SecureBank"</h2>
                <p style="font-size: 0.875rem; color: #64748b; margin-top: 0.25rem;">"Banking Platform"</p>
            </div>
            
            <nav style="display: flex; flex-direction: column; gap: 0.5rem; flex: 1;">
                <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                    <button
                        style=move || if is_active("/dashboard") { active_style } else { inactive_style }
                        on:click={
                        let navigate = navigate.clone();
                        move |_| { let _ = navigate("/dashboard", Default::default()); }
                    }
                >
                        <svg style="width: 20px; height: 20px; margin-right: 0.75rem;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2 2z"></path>
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 21l4-7 4 7"></path>
                        </svg>
                    "Dashboard"
                    </button>
                    
                    <button
                        style=move || if is_active("/accounts") { active_style } else { inactive_style }
                        on:click={
                        let navigate = navigate.clone();
                        move |_| { let _ = navigate("/accounts", Default::default()); }
                    }
                >
                        <svg style="width: 20px; height: 20px; margin-right: 0.75rem;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"></path>
                        </svg>
                    "Accounts"
                    </button>
                    
                    <button
                        style=move || if is_active("/transactions") { active_style } else { inactive_style }
                        on:click={
                        let navigate = navigate.clone();
                        move |_| { let _ = navigate("/transactions", Default::default()); }
                    }
                >
                        <svg style="width: 20px; height: 20px; margin-right: 0.75rem;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"></path>
                        </svg>
                    "Transactions"
                    </button>
                    
                    <button
                        style=move || if is_active("/transfer") { active_style } else { inactive_style }
                        on:click={
                        let navigate = navigate.clone();
                        move |_| { let _ = navigate("/transfer", Default::default()); }
                    }
                >
                        <svg style="width: 20px; height: 20px; margin-right: 0.75rem;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"></path>
                        </svg>
                    "Transfer"
                    </button>
                    
                    <button
                        style=move || if is_active("/deposit") { active_style } else { inactive_style }
                        on:click={
                        let navigate = navigate.clone();
                        move |_| { let _ = navigate("/deposit", Default::default()); }
                    }
                >
                        <svg style="width: 20px; height: 20px; margin-right: 0.75rem;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
                        </svg>
                    "Deposit"
                    </button>
                    
                    <button
                        style="display: none; align-items: center; width: 100%; padding: 0.875rem 1rem; border: none; background: rgba(255, 255, 255, 0.5); color: #64748b; border-radius: 0.75rem; font-weight: 500; font-size: 0.875rem; cursor: pointer; transition: all 0.2s; text-align: left;"
                        on:click={
                        let navigate = navigate.clone();
                        move |_| { let _ = navigate("/admin", Default::default()); }
                    }
                >
                        <svg style="width: 20px; height: 20px; margin-right: 0.75rem;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                        </svg>
                    "Admin"
                    </button>
                </div>
            </nav>
        </div>
    }
}

/// Dashboard content component with real account data
#[component]
fn DashboardContent() -> impl IntoView {
    let current_user = expect_context::<ReadSignal<Option<User>>>();
    let (accounts, set_accounts) = signal(Vec::<Account>::new());
    let (loading, set_loading) = signal(true);

    // Load user accounts
    Effect::new(move |_| {
        if let Some(user) = current_user.get() {
            spawn_local(async move {
                set_loading.set(true);
                match get_accounts_by_user(user.id).await {
                    Ok(user_accounts) => {
                        set_accounts.set(user_accounts);
                    }
                    Err(e) => {
                        leptos::logging::log!("Failed to load accounts: {}", e);
                    }
                }
                set_loading.set(false);
            });
        }
    });

    let total_balance = move || {
        accounts.get().iter()
            .map(|acc| acc.balance)
            .fold(rust_decimal::Decimal::ZERO, |sum, balance| sum + balance)
    };

    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem;">
            // Stats cards
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1.5rem;">
                <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 1.5rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                    <div style="display: flex; align-items: center; justify-content: space-between;">
                            <div>
                            <p style="font-size: 0.875rem; font-weight: 600; color: #64748b; margin-bottom: 0.5rem;">"Total Balance"</p>
                            <p style="font-size: 2.5rem; font-weight: 700; color: #1e293b; margin: 0;">
                                    "$" {move || format!("{:.2}", total_balance())}
                                </p>
                            </div>
                        <div style="width: 60px; height: 60px; background: linear-gradient(135deg, #10b981 0%, #059669 100%); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                            <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1"></path>
                            </svg>
                        </div>
                    </div>
                </div>
                
                <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 1.5rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                    <div style="display: flex; align-items: center; justify-content: space-between;">
                            <div>
                            <p style="font-size: 0.875rem; font-weight: 600; color: #64748b; margin-bottom: 0.5rem;">"Active Accounts"</p>
                            <p style="font-size: 2.5rem; font-weight: 700; color: #1e293b; margin: 0;">
                                    {move || accounts.get().len()}
                                </p>
                            </div>
                        <div style="width: 60px; height: 60px; background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                            <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
                            </svg>
                        </div>
                    </div>
                </div>
                
                <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 1.5rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                    <div style="display: flex; align-items: center; justify-content: space-between;">
                            <div>
                            <p style="font-size: 0.875rem; font-weight: 600; color: #64748b; margin-bottom: 0.5rem;">"Account Status"</p>
                            <p style="font-size: 1.5rem; font-weight: 600; color: #10b981; margin: 0;">"Active"</p>
                            </div>
                        <div style="width: 60px; height: 60px; background: linear-gradient(135deg, #10b981 0%, #059669 100%); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                            <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                        </div>
                    </div>
                </div>
            </div>

            // Recent accounts card
            <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 2rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                <h3 style="font-size: 1.25rem; font-weight: 700; color: #1e293b; margin-bottom: 1.5rem; display: flex; align-items: center;">
                    <svg style="width: 24px; height: 24px; margin-right: 0.5rem; color: #7c3aed;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"></path>
                    </svg>
                    "Recent Accounts"
                </h3>
                    {move || {
                        if loading.get() {
                        view! { 
                            <div style="text-align: center; padding: 2rem; color: #64748b;">
                                <div style="display: inline-flex; align-items: center; justify-content: center; width: 40px; height: 40px; margin-bottom: 1rem;">
                                    <svg style="width: 24px; height: 24px; color: #7c3aed;" class="animate-spin" fill="none" viewBox="0 0 24 24">
                                        <circle style="opacity: 0.25;" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path style="opacity: 0.75;" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                    </svg>
                                </div>
                                <p>"Loading accounts..."</p>
                            </div> 
                        }.into_any()
                        } else if accounts.get().is_empty() {
                        view! { 
                            <div style="text-align: center; padding: 2rem; color: #64748b;">
                                <div style="display: inline-flex; align-items: center; justify-content: center; width: 60px; height: 60px; background: #f1f5f9; border-radius: 50%; margin-bottom: 1rem;">
                                    <svg style="width: 24px; height: 24px; color: #64748b;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 009.586 13H7"></path>
                                    </svg>
                                </div>
                                <p>"No accounts found"</p>
                            </div> 
                        }.into_any()
                        } else {
                            view! {
                            <div style="display: flex; flex-direction: column; gap: 1rem;">
                                    <For
                                        each=move || accounts.get()
                                        key=|account| account.id
                                        children=move |account| {
                                            view! {
                                            <div style="display: flex; justify-content: space-between; align-items: center; padding: 1.5rem; background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%); border: 1px solid #e2e8f0; border-radius: 0.75rem; transition: all 0.2s;">
                                                <div style="display: flex; align-items: center;">
                                                    <div style="width: 48px; height: 48px; background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%); border-radius: 0.75rem; display: flex; align-items: center; justify-content: center; margin-right: 1rem;">
                                                        <svg style="width: 20px; height: 20px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"></path>
                                                        </svg>
                                                    </div>
                                                    <div>
                                                        <p style="font-weight: 600; color: #1e293b; margin-bottom: 0.25rem;">{account.account_type.to_string().to_uppercase()} " Account"</p>
                                                        <p style="font-size: 0.875rem; color: #64748b; margin: 0;">"Account #: " {account.account_number.clone()}</p>
                                                    </div>
                                                </div>
                                                <div style="text-align: right;">
                                                    <p style="font-weight: 700; font-size: 1.25rem; color: #1e293b; margin-bottom: 0.25rem;">"$" {format!("{:.2}", account.balance)}</p>
                                                    <p style="font-size: 0.875rem; color: #64748b; margin: 0;">{account.currency.clone()}</p>
                                                    </div>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
        </div>
    }
}

/// Accounts page component with real data
#[component]
fn AccountsPage() -> impl IntoView {
    let current_user = expect_context::<ReadSignal<Option<User>>>();
    let auth_token = expect_context::<ReadSignal<Option<String>>>();
    let (accounts, set_accounts) = signal(Vec::<Account>::new());
    let (loading, set_loading) = signal(true);
    let navigate = use_navigate();
    
    // Redirect if not authenticated
    Effect::new(move |_| {
        if auth_token.get().is_none() {
            let _ = navigate("/", Default::default());
        }
    });

    // Load user accounts
    Effect::new(move |_| {
        if let Some(user) = current_user.get() {
            spawn_local(async move {
                set_loading.set(true);
                match get_accounts_by_user(user.id).await {
                    Ok(user_accounts) => {
                        set_accounts.set(user_accounts);
                    }
                    Err(e) => {
                        leptos::logging::log!("Failed to load accounts: {}", e);
                    }
                }
                set_loading.set(false);
            });
        }
    });

    let total_balance = move || {
        accounts.get().iter()
            .map(|acc| acc.balance)
            .fold(rust_decimal::Decimal::ZERO, |sum, balance| sum + balance)
    };

    view! {
        <div style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); position: relative; overflow: hidden;">
            // Background elements
            <div style="position: absolute; inset: 0; overflow: hidden; pointer-events: none;">
                <div style="position: absolute; top: -50px; right: -50px; width: 120px; height: 120px; background: rgba(255, 255, 255, 0.1); border-radius: 50%; filter: blur(20px);" class="animate-blob"></div>
                <div style="position: absolute; bottom: -30px; left: -30px; width: 100px; height: 100px; background: rgba(255, 255, 255, 0.08); border-radius: 50%; filter: blur(25px);" class="animate-blob animation-delay-2000"></div>
                <div style="position: absolute; top: 50%; right: 20px; width: 80px; height: 80px; background: rgba(255, 255, 255, 0.06); border-radius: 50%; filter: blur(30px);" class="animate-blob animation-delay-4000"></div>
            </div>
            
            <div style="display: flex; min-height: 100vh; position: relative; z-index: 10;">
                // Sidebar
                <div style="width: 280px; background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border-right: 1px solid rgba(255, 255, 255, 0.2); box-shadow: 5px 0 25px rgba(0, 0, 0, 0.1);">
                <NavigationSidebar />
                </div>
                
                // Main content
                <div style="flex: 1; display: flex; flex-direction: column;">
                    // Header
                    <div style="background: rgba(255, 255, 255, 0.9); backdrop-filter: blur(10px); border-bottom: 1px solid rgba(255, 255, 255, 0.2); padding: 1.5rem 2rem; box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);">
                        <div style="display: flex; justify-content: space-between; align-items: center;">
                            <div style="display: flex; align-items: center;">
                                <div style="display: inline-flex; align-items: center; justify-content: center; width: 48px; height: 48px; background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%); border-radius: 0.75rem; margin-right: 1rem;">
                                    <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"></path>
                                    </svg>
                                </div>
                                <div>
                                    <h1 style="font-size: 2rem; font-weight: 700; color: #1e293b; margin: 0; letter-spacing: -0.025em;">"My Accounts"</h1>
                                    <p style="font-size: 0.875rem; color: #64748b; margin: 0;">"Manage and view all your bank accounts"</p>
                                </div>
                            </div>
                            <div style="display: flex; align-items: center; gap: 1rem;">
                                {move || current_user.get().map(|user| view! {
                                    <span style="color: #64748b; font-weight: 500;">"Welcome, " {user.first_name} " " {user.last_name}</span>
                                })}
                        <LogoutButton />
                    </div>
                        </div>
                    </div>
                    
                    // Content area
                    <div style="flex: 1; padding: 2rem; background: transparent;">
                        <div style="max-width: 1200px; margin: 0 auto; display: flex; flex-direction: column; gap: 2rem;">
                            // Summary Cards
                            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 1.5rem;">
                                <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 1.5rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                                    <div style="display: flex; align-items: center; justify-content: space-between;">
                                        <div>
                                            <p style="font-size: 0.875rem; font-weight: 600; color: #64748b; margin-bottom: 0.5rem;">"Total Balance"</p>
                                            <p style="font-size: 2rem; font-weight: 700; color: #1e293b; margin: 0;">
                                                "$" {move || format!("{:.2}", total_balance())}
                                            </p>
                                        </div>
                                        <div style="width: 50px; height: 50px; background: linear-gradient(135deg, #10b981 0%, #059669 100%); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                                            <svg style="width: 20px; height: 20px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1"></path>
                                            </svg>
                                        </div>
                                    </div>
                                </div>
                                
                                <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 1.5rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                                    <div style="display: flex; align-items: center; justify-content: space-between;">
                                        <div>
                                            <p style="font-size: 0.875rem; font-weight: 600; color: #64748b; margin-bottom: 0.5rem;">"Active Accounts"</p>
                                            <p style="font-size: 2rem; font-weight: 700; color: #1e293b; margin: 0;">
                                                {move || accounts.get().len()}
                                            </p>
                                        </div>
                                        <div style="width: 50px; height: 50px; background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                                            <svg style="width: 20px; height: 20px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
                                            </svg>
                                        </div>
                                    </div>
                                </div>
                                
                                <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 1.5rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                                    <div style="display: flex; align-items: center; justify-content: space-between;">
                                        <div>
                                            <p style="font-size: 0.875rem; font-weight: 600; color: #64748b; margin-bottom: 0.5rem;">"Account Types"</p>
                                            <p style="font-size: 1.25rem; font-weight: 600; color: #1e293b; margin: 0;">
                                                {move || {
                                                    let types: std::collections::HashSet<String> = accounts.get().iter()
                                                        .map(|acc| acc.account_type.to_string())
                                                        .collect();
                                                    types.len()
                                                }}
                                            </p>
                                        </div>
                                        <div style="width: 50px; height: 50px; background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                                            <svg style="width: 20px; height: 20px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                                            </svg>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            
                            // Accounts List
                            <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 2rem; box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);">
                                <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 2rem;">
                                    <div style="display: flex; align-items: center;">
                                        <div style="display: inline-flex; align-items: center; justify-content: center; width: 48px; height: 48px; background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%); border-radius: 0.75rem; margin-right: 1rem;">
                                            <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"></path>
                                            </svg>
                                        </div>
                                        <div>
                                            <h2 style="font-size: 1.5rem; font-weight: 700; color: #1e293b; margin: 0;">"Your Accounts"</h2>
                                            <p style="color: #64748b; margin: 0; font-size: 0.875rem;">"Detailed view of all your bank accounts"</p>
                                        </div>
                                    </div>
                                    <div style="background: linear-gradient(135deg, #10b981 0%, #059669 100%); color: white; padding: 0.5rem 1rem; border-radius: 0.5rem; font-size: 0.875rem; font-weight: 600;">
                                        {move || format!("{} accounts", accounts.get().len())}
                                    </div>
                                </div>
                                
                    {move || {
                        if loading.get() {
                                        view! { 
                                            <div style="text-align: center; padding: 4rem 2rem; color: #64748b;">
                                                <div style="display: inline-flex; align-items: center; justify-content: center; width: 60px; height: 60px; margin-bottom: 1.5rem;">
                                                    <svg style="width: 32px; height: 32px; color: #7c3aed;" class="animate-spin" fill="none" viewBox="0 0 24 24">
                                                        <circle style="opacity: 0.25;" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                                        <path style="opacity: 0.75;" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                                    </svg>
                                                </div>
                                                <h3 style="font-size: 1.25rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem;">"Loading Your Accounts"</h3>
                                                <p style="margin: 0;">"Please wait while we retrieve your account information..."</p>
                                            </div> 
                                        }.into_any()
                        } else if accounts.get().is_empty() {
                                        view! { 
                                            <div style="text-align: center; padding: 4rem 2rem; color: #64748b;">
                                                <div style="display: inline-flex; align-items: center; justify-content: center; width: 80px; height: 80px; background: linear-gradient(135deg, #f1f5f9 0%, #e2e8f0 100%); border-radius: 50%; margin-bottom: 2rem;">
                                                    <svg style="width: 32px; height: 32px; color: #64748b;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 009.586 13H7"></path>
                                                    </svg>
                                                </div>
                                                <h3 style="font-size: 1.5rem; font-weight: 600; color: #374151; margin-bottom: 1rem;">"No Accounts Found"</h3>
                                                <p style="margin-bottom: 2rem; font-size: 1rem;">"You don't have any bank accounts yet. Contact your bank to open a new account."</p>
                                                <div style="background: linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%); border: 1px solid #93c5fd; color: #1e40af; padding: 1rem; border-radius: 0.75rem; text-align: left; max-width: 400px; margin: 0 auto;">
                                                    <p style="font-weight: 600; margin-bottom: 0.5rem;">"Need help?"</p>
                                                    <p style="margin: 0; font-size: 0.875rem;">"Contact our customer service team to open your first account or if you believe this is an error."</p>
                                                </div>
                                            </div> 
                                        }.into_any()
                        } else {
                            view! {
                                            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(400px, 1fr)); gap: 1.5rem;">
                                    <For
                                        each=move || accounts.get()
                                        key=|account| account.id
                                        children=move |account| {
                                                        let account_type_color = match account.account_type.to_string().as_str() {
                                                            "checking" => "linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%)",
                                                            "savings" => "linear-gradient(135deg, #10b981 0%, #059669 100%)",
                                                            "business" => "linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%)",
                                                            _ => "linear-gradient(135deg, #6b7280 0%, #4b5563 100%)",
                                                        };
                                                        
                                            view! {
                                                            <div style="background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%); border: 1px solid #e2e8f0; border-radius: 1rem; padding: 2rem; transition: all 0.3s; position: relative; overflow: hidden;">
                                                                // Account Type Badge
                                                                <div style="position: absolute; top: 1rem; right: 1rem;">
                                                                    <div style=format!("background: {}; color: white; padding: 0.5rem 1rem; border-radius: 1rem; font-size: 0.75rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em;", account_type_color)>
                                                                        {account.account_type.to_string()}
                                                                    </div>
                                                                </div>
                                                                
                                                                // Account Icon and Info
                                                                <div style="display: flex; align-items: flex-start; margin-bottom: 1.5rem;">
                                                                    <div style=format!("width: 60px; height: 60px; background: {}; border-radius: 1rem; display: flex; align-items: center; justify-content: center; margin-right: 1.5rem; flex-shrink: 0;", account_type_color)>
                                                                        <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"></path>
                                                                        </svg>
                                                                    </div>
                                                                    <div style="flex: 1; min-width: 0;">
                                                                        <h3 style="font-size: 1.25rem; font-weight: 700; color: #1e293b; margin-bottom: 0.5rem; text-transform: capitalize;">
                                                                            {account.account_type.to_string()} " Account"
                                                        </h3>
                                                                        <p style="font-size: 0.875rem; color: #64748b; margin: 0; font-family: monospace; background: rgba(255, 255, 255, 0.8); padding: 0.25rem 0.5rem; border-radius: 0.25rem; display: inline-block;">
                                                            "Account #: " {account.account_number.clone()}
                                                        </p>
                                                                    </div>
                                                                </div>
                                                                
                                                                // Balance Section
                                                                <div style="background: rgba(255, 255, 255, 0.8); border-radius: 0.75rem; padding: 1.5rem; margin-bottom: 1.5rem;">
                                                                    <div style="display: flex; justify-content: space-between; align-items: center;">
                                                                        <div>
                                                                            <p style="font-size: 0.875rem; color: #64748b; margin-bottom: 0.5rem; font-weight: 500;">"Current Balance"</p>
                                                                            <p style="font-size: 2rem; font-weight: 700; color: #1e293b; margin: 0; font-family: monospace;">
                                                            "$" {format!("{:.2}", account.balance)}
                                                        </p>
                                                                        </div>
                                                                        <div style="text-align: right;">
                                                                            <p style="font-size: 0.875rem; color: #64748b; margin-bottom: 0.5rem; font-weight: 500;">"Currency"</p>
                                                                            <p style="font-size: 1rem; font-weight: 600; color: #374151; margin: 0;">
                                                            {account.currency.clone()}
                                                        </p>
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                                
                                                                // Status and Actions
                                                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                                                    <div style="display: flex; align-items: center;">
                                                                        <div style=format!("width: 8px; height: 8px; border-radius: 50%; margin-right: 0.5rem; background: {};", 
                                                                            if account.is_active { "#10b981" } else { "#ef4444" })>
                                                                        </div>
                                                                        <span style=format!("font-size: 0.875rem; font-weight: 600; color: {};", 
                                                                            if account.is_active { "#10b981" } else { "#ef4444" })>
                                                                {if account.is_active { "Active" } else { "Inactive" }}
                                                            </span>
                                                        </div>
                                                                    <div style="font-size: 0.75rem; color: #64748b;">
                                                                        "Created: " {account.created_at.format("%b %d, %Y").to_string()}
                                                    </div>
                                                                </div>
                                                            </div>
                                            }
                                        }
                                    />
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Transactions page component with real data
#[component]
fn TransactionsPage() -> impl IntoView {
    let current_user = expect_context::<ReadSignal<Option<User>>>();
    let auth_token = expect_context::<ReadSignal<Option<String>>>();
    let (transactions, set_transactions) = signal(Vec::<Transaction>::new());
    let (loading, set_loading) = signal(true);
    let navigate = use_navigate();
    
    // Redirect if not authenticated
    Effect::new(move |_| {
        if auth_token.get().is_none() {
            let _ = navigate("/", Default::default());
        }
    });

    // Load transactions
    Effect::new(move |_| {
        if current_user.get().is_some() {
            spawn_local(async move {
                set_loading.set(true);
                match get_transactions().await {
                    Ok(all_transactions) => {
                        set_transactions.set(all_transactions);
                    }
                    Err(e) => {
                        leptos::logging::log!("Failed to load transactions: {}", e);
                    }
                }
                set_loading.set(false);
            });
        }
    });

    // Calculate statistics
    let total_volume = move || {
        transactions.get().iter()
            .map(|txn| txn.amount)
            .fold(rust_decimal::Decimal::ZERO, |sum, amount| sum + amount)
    };

    let pending_count = move || {
        transactions.get().iter()
            .filter(|txn| txn.status == TransactionStatus::Pending)
            .count()
    };

    let completed_count = move || {
        transactions.get().iter()
            .filter(|txn| txn.status == TransactionStatus::Completed)
            .count()
    };

    view! {
        <div style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); position: relative; overflow: hidden;">
            // Background elements
            <div style="position: absolute; inset: 0; overflow: hidden; pointer-events: none;">
                <div style="position: absolute; top: -50px; right: -50px; width: 120px; height: 120px; background: rgba(255, 255, 255, 0.1); border-radius: 50%; filter: blur(20px);" class="animate-blob"></div>
                <div style="position: absolute; bottom: -30px; left: -30px; width: 100px; height: 100px; background: rgba(255, 255, 255, 0.08); border-radius: 50%; filter: blur(25px);" class="animate-blob animation-delay-2000"></div>
                <div style="position: absolute; top: 50%; right: 20px; width: 80px; height: 80px; background: rgba(255, 255, 255, 0.06); border-radius: 50%; filter: blur(30px);" class="animate-blob animation-delay-4000"></div>
            </div>
            
            <div style="display: flex; min-height: 100vh; position: relative; z-index: 10;">
                // Sidebar
                <div style="width: 280px; background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border-right: 1px solid rgba(255, 255, 255, 0.2); box-shadow: 5px 0 25px rgba(0, 0, 0, 0.1);">
                <NavigationSidebar />
                </div>
                
                // Main content
                <div style="flex: 1; display: flex; flex-direction: column;">
                    // Header
                    <div style="background: rgba(255, 255, 255, 0.9); backdrop-filter: blur(10px); border-bottom: 1px solid rgba(255, 255, 255, 0.2); padding: 1.5rem 2rem; box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);">
                        <div style="display: flex; justify-content: space-between; align-items: center;">
                            <div style="display: flex; align-items: center;">
                                <div style="display: inline-flex; align-items: center; justify-content: center; width: 48px; height: 48px; background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%); border-radius: 0.75rem; margin-right: 1rem;">
                                    <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"></path>
                                    </svg>
                                </div>
                                <div>
                                    <h1 style="font-size: 2rem; font-weight: 700; color: #1e293b; margin: 0; letter-spacing: -0.025em;">"Transaction History"</h1>
                                    <p style="font-size: 0.875rem; color: #64748b; margin: 0;">"Track all your financial transactions"</p>
                                </div>
                            </div>
                            <div style="display: flex; align-items: center; gap: 1rem;">
                                {move || current_user.get().map(|user| view! {
                                    <span style="color: #64748b; font-weight: 500;">"Welcome, " {user.first_name} " " {user.last_name}</span>
                                })}
                        <LogoutButton />
                    </div>
                        </div>
                    </div>
                    
                    // Content area
                    <div style="flex: 1; padding: 2rem; background: transparent;">
                        <div style="max-width: 1200px; margin: 0 auto; display: flex; flex-direction: column; gap: 2rem;">
                            // Statistics Cards
                            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem;">
                                <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 1.5rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                                    <div style="display: flex; align-items: center; justify-content: space-between;">
                                        <div>
                                            <p style="font-size: 0.875rem; font-weight: 600; color: #64748b; margin-bottom: 0.5rem;">"Total Transactions"</p>
                                            <p style="font-size: 2rem; font-weight: 700; color: #1e293b; margin: 0;">
                                                {move || transactions.get().len()}
                                            </p>
                                        </div>
                                        <div style="width: 50px; height: 50px; background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                                            <svg style="width: 20px; height: 20px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"></path>
                                            </svg>
                                        </div>
                                    </div>
                                </div>
                                
                                <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 1.5rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                                    <div style="display: flex; align-items: center; justify-content: space-between;">
                                        <div>
                                            <p style="font-size: 0.875rem; font-weight: 600; color: #64748b; margin-bottom: 0.5rem;">"Total Volume"</p>
                                            <p style="font-size: 1.5rem; font-weight: 700; color: #1e293b; margin: 0;">
                                                "$" {move || format!("{:.2}", total_volume())}
                                            </p>
                                        </div>
                                        <div style="width: 50px; height: 50px; background: linear-gradient(135deg, #10b981 0%, #059669 100%); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                                            <svg style="width: 20px; height: 20px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1"></path>
                                            </svg>
                                        </div>
                                    </div>
                                </div>
                                
                                <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 1.5rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                                    <div style="display: flex; align-items: center; justify-content: space-between;">
                                        <div>
                                            <p style="font-size: 0.875rem; font-weight: 600; color: #64748b; margin-bottom: 0.5rem;">"Completed"</p>
                                            <p style="font-size: 2rem; font-weight: 700; color: #10b981; margin: 0;">
                                                {move || completed_count()}
                                            </p>
                                        </div>
                                        <div style="width: 50px; height: 50px; background: linear-gradient(135deg, #10b981 0%, #059669 100%); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                                            <svg style="width: 20px; height: 20px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                            </svg>
                                        </div>
                                    </div>
                                </div>
                                
                                <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 1.5rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                                    <div style="display: flex; align-items: center; justify-content: space-between;">
                                        <div>
                                            <p style="font-size: 0.875rem; font-weight: 600; color: #64748b; margin-bottom: 0.5rem;">"Pending"</p>
                                            <p style="font-size: 2rem; font-weight: 700; color: #f59e0b; margin: 0;">
                                                {move || pending_count()}
                                            </p>
                                        </div>
                                        <div style="width: 50px; height: 50px; background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                                            <svg style="width: 20px; height: 20px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                            </svg>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            
                            // Transactions List
                            <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 2rem; box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);">
                                <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 2rem;">
                                    <div style="display: flex; align-items: center;">
                                        <div style="display: inline-flex; align-items: center; justify-content: center; width: 48px; height: 48px; background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%); border-radius: 0.75rem; margin-right: 1rem;">
                                            <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"></path>
                                            </svg>
                                        </div>
                                        <div>
                                            <h2 style="font-size: 1.5rem; font-weight: 700; color: #1e293b; margin: 0;">"Recent Transactions"</h2>
                                            <p style="color: #64748b; margin: 0; font-size: 0.875rem;">"Your latest financial activity"</p>
                                        </div>
                                    </div>
                                    <div style="background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%); color: white; padding: 0.5rem 1rem; border-radius: 0.5rem; font-size: 0.875rem; font-weight: 600;">
                                        {move || format!("{} transactions", transactions.get().len())}
                                    </div>
                                </div>
                                
                            {move || {
                                if loading.get() {
                                        view! { 
                                            <div style="text-align: center; padding: 4rem 2rem; color: #64748b;">
                                                <div style="display: inline-flex; align-items: center; justify-content: center; width: 60px; height: 60px; margin-bottom: 1.5rem;">
                                                    <svg style="width: 32px; height: 32px; color: #7c3aed;" class="animate-spin" fill="none" viewBox="0 0 24 24">
                                                        <circle style="opacity: 0.25;" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                                        <path style="opacity: 0.75;" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                                    </svg>
                                                </div>
                                                <h3 style="font-size: 1.25rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem;">"Loading Transactions"</h3>
                                                <p style="margin: 0;">"Please wait while we retrieve your transaction history..."</p>
                                            </div> 
                                        }.into_any()
                                } else if transactions.get().is_empty() {
                                        view! { 
                                            <div style="text-align: center; padding: 4rem 2rem; color: #64748b;">
                                                <div style="display: inline-flex; align-items: center; justify-content: center; width: 80px; height: 80px; background: linear-gradient(135deg, #f1f5f9 0%, #e2e8f0 100%); border-radius: 50%; margin-bottom: 2rem;">
                                                    <svg style="width: 32px; height: 32px; color: #64748b;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"></path>
                                                    </svg>
                                                </div>
                                                <h3 style="font-size: 1.5rem; font-weight: 600; color: #374151; margin-bottom: 1rem;">"No Transactions Found"</h3>
                                                <p style="margin-bottom: 2rem; font-size: 1rem;">"You haven't made any transactions yet. Start by transferring money or making a payment."</p>
                                                <div style="background: linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%); border: 1px solid #93c5fd; color: #1e40af; padding: 1rem; border-radius: 0.75rem; text-align: left; max-width: 400px; margin: 0 auto;">
                                                    <p style="font-weight: 600; margin-bottom: 0.5rem;">"Getting Started:"</p>
                                                    <p style="margin: 0; font-size: 0.875rem;">"Visit the Transfer page to send money or check your account balance on the Dashboard."</p>
                                                </div>
                                            </div> 
                                        }.into_any()
                                } else {
                                    view! {
                                            <div style="display: flex; flex-direction: column; gap: 1rem;">
                                                <For
                                                    each=move || transactions.get()
                                                    key=|txn| txn.id
                                                    children=move |txn| {
                                                        let status_color = match txn.status {
                                                            TransactionStatus::Completed => "#10b981",
                                                            TransactionStatus::Pending => "#f59e0b", 
                                                            TransactionStatus::Failed => "#ef4444",
                                                            TransactionStatus::Cancelled => "#6b7280",
                                                        };
                                                        
                                                        let type_color = match txn.transaction_type {
                                                            TransactionType::Deposit => "#10b981",
                                                            TransactionType::Withdrawal => "#ef4444",
                                                            TransactionType::Transfer => "#3b82f6",
                                                            TransactionType::Payment => "#8b5cf6",
                                                        };
                                                        
                                                        let type_icon = match txn.transaction_type {
                                                            TransactionType::Deposit => "M7 16l-4-4m0 0l4-4m-4 4h18",
                                                            TransactionType::Withdrawal => "M17 8l4 4m0 0l-4 4m4-4H3",
                                                            TransactionType::Transfer => "M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4",
                                                            TransactionType::Payment => "M17 8l4 4m0 0l-4 4m4-4H3",
                                                        };
                                                        
                                                        view! {
                                                            <div style="background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%); border: 1px solid #e2e8f0; border-radius: 0.75rem; padding: 1.5rem; transition: all 0.2s; hover:shadow-lg;">
                                                                <div style="display: flex; justify-content: space-between; align-items: flex-start;">
                                                                    <div style="display: flex; align-items: center; flex: 1;">
                                                                        // Transaction Type Icon
                                                                        <div style=format!("width: 48px; height: 48px; background: {}; border-radius: 0.75rem; display: flex; align-items: center; justify-content: center; margin-right: 1rem; opacity: 0.1; position: relative;", type_color)>
                                                                            <div style=format!("position: absolute; inset: 0; background: {}; border-radius: 0.75rem; opacity: 0.2;", type_color)></div>
                                                                            <svg style=format!("width: 20px; height: 20px; color: {}; position: relative; z-index: 1;", type_color) fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=type_icon></path>
                                                                            </svg>
                                                                        </div>
                                                                        
                                                                        // Transaction Details
                                                                        <div style="flex: 1; min-width: 0;">
                                                                            <div style="display: flex; align-items: center; margin-bottom: 0.5rem;">
                                                                                <h3 style="font-size: 1rem; font-weight: 600; color: #1e293b; margin: 0; margin-right: 0.75rem; text-transform: capitalize;">
                                                                                    {txn.transaction_type.to_string()}
                                                                                </h3>
                                                                                <div style=format!("background: {}; color: white; padding: 0.25rem 0.5rem; border-radius: 0.5rem; font-size: 0.75rem; font-weight: 600; text-transform: uppercase;", status_color)>
                                                                                    {txn.status.to_string()}
                                                                                </div>
                                                                            </div>
                                                                            
                                                                            <p style="font-size: 0.875rem; color: #64748b; margin-bottom: 0.5rem;">
                                                                                {txn.description.clone().unwrap_or_else(|| "No description provided".to_string())}
                                                                            </p>
                                                                            
                                                                            <div style="display: flex; align-items: center; gap: 1rem; font-size: 0.75rem; color: #9ca3af;">
                                                                                <span style="font-family: monospace;">
                                                                                    "Ref: " {txn.reference_number.clone()}
                                                                                </span>
                                                                                <span>
                                                                                    {txn.created_at.format("%b %d, %Y at %H:%M").to_string()}
                                                                                </span>
                                                                            </div>
                                                                        </div>
                                                                    </div>
                                                                    
                                                                    // Amount
                                                                    <div style="text-align: right; margin-left: 1rem;">
                                                                        <p style=format!("font-size: 1.25rem; font-weight: 700; margin: 0; font-family: monospace; color: {};", 
                                                                            match txn.transaction_type {
                                                                                TransactionType::Deposit => "#10b981",
                                                                                TransactionType::Withdrawal => "#ef4444", 
                                                                                TransactionType::Transfer => "#1e293b",
                                                                                TransactionType::Payment => "#8b5cf6",
                                                                            })>
                                                                            {match txn.transaction_type {
                                                                                TransactionType::Deposit => "+",
                                                                                TransactionType::Withdrawal => "-",
                                                                                TransactionType::Transfer => "",
                                                                                TransactionType::Payment => "-",
                                                                            }}
                                                                    "$" {format!("{:.2}", txn.amount)}
                                                                        </p>
                                                                        
                                                                        // Account info if available
                                                                        <div style="font-size: 0.75rem; color: #9ca3af; margin-top: 0.5rem;">
                                                                            {if let (Some(from_id), Some(to_id)) = (txn.from_account_id, txn.to_account_id) {
                                                                                view! {
                                                                                    <div>
                                                                                        <div>"From: " {from_id}</div>
                                                                                        <div>"To: " {to_id}</div>
                                                                                    </div>
                                                                                }.into_any()
                                                                            } else if let Some(from_id) = txn.from_account_id {
                                                                                view! { <div>"Account: " {from_id}</div> }.into_any()
                                                                            } else if let Some(to_id) = txn.to_account_id {
                                                                                view! { <div>"Account: " {to_id}</div> }.into_any()
                                                                            } else {
                                                                                view! { <div></div> }.into_any()
                                                                            }}
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        }
                                                    }
                                                />
                                            </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Transfer page component with real functionality
#[component]
fn TransferPage() -> impl IntoView {
    let current_user = expect_context::<ReadSignal<Option<User>>>();
    let auth_token = expect_context::<ReadSignal<Option<String>>>();
    let (accounts, set_accounts) = signal(Vec::<Account>::new());
    let (from_account_id, set_from_account_id) = signal(0i32);
    let (to_account_number, set_to_account_number) = signal(String::new());
    let (amount, set_amount) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (success, set_success) = signal(Option::<String>::None);
    let (error, set_error) = signal(Option::<String>::None);
    let navigate = use_navigate();
    
    // Redirect if not authenticated
    Effect::new(move |_| {
        if auth_token.get().is_none() {
            let _ = navigate("/", Default::default());
        }
    });

    // Load user accounts
    Effect::new(move |_| {
        if let Some(user) = current_user.get() {
            spawn_local(async move {
                match get_accounts_by_user(user.id).await {
                    Ok(user_accounts) => {
                        set_accounts.set(user_accounts.clone());
                        if let Some(first_account) = user_accounts.first() {
                            set_from_account_id.set(first_account.id);
                        }
                    }
                    Err(e) => {
                        leptos::logging::log!("Failed to load accounts: {}", e);
                    }
                }
            });
        }
    });

    let transfer_action = Action::new(move |_: &()| {
        let from_account_id = from_account_id.get();
        let to_account_number = to_account_number.get();
        let amount_str = amount.get();
        let description = description.get();
        
        async move {
            set_loading.set(true);
            set_error.set(None);
            set_success.set(None);
            
            // Validation
            if to_account_number.is_empty() {
                set_error.set(Some("Please enter destination account number".to_string()));
                set_loading.set(false);
                return;
            }
            
            if amount_str.is_empty() {
                set_error.set(Some("Please enter transfer amount".to_string()));
                set_loading.set(false);
                return;
            }
            
            // Parse amount
            let amount_decimal = match amount_str.parse::<rust_decimal::Decimal>() {
                Ok(amt) => {
                    if amt <= rust_decimal::Decimal::ZERO {
                        set_error.set(Some("Amount must be greater than zero".to_string()));
                        set_loading.set(false);
                        return;
                    }
                    amt
                },
                Err(_) => {
                    set_error.set(Some("Invalid amount format".to_string()));
                    set_loading.set(false);
                    return;
                }
            };
            
            let request = TransferRequest {
                from_account_id,
                to_account_number,
                amount: amount_decimal,
                description: if description.is_empty() { None } else { Some(description) },
            };
            
            match create_transfer(request).await {
                Ok(_) => {
                    set_success.set(Some("Transfer completed successfully!".to_string()));
                    set_to_account_number.set(String::new());
                    set_amount.set(String::new());
                    set_description.set(String::new());
                }
                Err(e) => {
                    set_error.set(Some(format!("Transfer failed: {}", e)));
                }
            }
            set_loading.set(false);
        }
    });

    let on_transfer = move |_| {
        transfer_action.dispatch(());
    };

    view! {
        <div style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); position: relative; overflow: hidden;">
            // Background elements
            <div style="position: absolute; inset: 0; overflow: hidden; pointer-events: none;">
                <div style="position: absolute; top: -50px; right: -50px; width: 120px; height: 120px; background: rgba(255, 255, 255, 0.1); border-radius: 50%; filter: blur(20px);" class="animate-blob"></div>
                <div style="position: absolute; bottom: -30px; left: -30px; width: 100px; height: 100px; background: rgba(255, 255, 255, 0.08); border-radius: 50%; filter: blur(25px);" class="animate-blob animation-delay-2000"></div>
                <div style="position: absolute; top: 50%; right: 20px; width: 80px; height: 80px; background: rgba(255, 255, 255, 0.06); border-radius: 50%; filter: blur(30px);" class="animate-blob animation-delay-4000"></div>
            </div>
            
            <div style="display: flex; min-height: 100vh; position: relative; z-index: 10;">
                // Sidebar
                <div style="width: 280px; background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border-right: 1px solid rgba(255, 255, 255, 0.2); box-shadow: 5px 0 25px rgba(0, 0, 0, 0.1);">
                <NavigationSidebar />
                </div>
                
                // Main content
                <div style="flex: 1; display: flex; flex-direction: column;">
                    // Header
                    <div style="background: rgba(255, 255, 255, 0.9); backdrop-filter: blur(10px); border-bottom: 1px solid rgba(255, 255, 255, 0.2); padding: 1.5rem 2rem; box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);">
                        <div style="display: flex; justify-content: space-between; align-items: center;">
                            <div style="display: flex; align-items: center;">
                                <div style="display: inline-flex; align-items: center; justify-content: center; width: 48px; height: 48px; background: linear-gradient(135deg, #10b981 0%, #059669 100%); border-radius: 0.75rem; margin-right: 1rem;">
                                    <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"></path>
                                    </svg>
                                </div>
                                <div>
                                    <h1 style="font-size: 2rem; font-weight: 700; color: #1e293b; margin: 0; letter-spacing: -0.025em;">"Transfer Money"</h1>
                                    <p style="font-size: 0.875rem; color: #64748b; margin: 0;">"Send money to another account securely"</p>
                                </div>
                            </div>
                            <div style="display: flex; align-items: center; gap: 1rem;">
                                {move || current_user.get().map(|user| view! {
                                    <span style="color: #64748b; font-weight: 500;">"Welcome, " {user.first_name} " " {user.last_name}</span>
                                })}
                        <LogoutButton />
                    </div>
                        </div>
                    </div>
                    
                    // Content area
                    <div style="flex: 1; padding: 2rem; background: transparent;">
                        <div style="max-width: 800px; margin: 0 auto;">
                            // Success/Error Messages
                            {move || success.get().map(|msg| view! {
                                <div style="background: rgba(220, 252, 231, 0.95); backdrop-filter: blur(10px); border: 1px solid #86efac; color: #166534; padding: 1rem; border-radius: 0.75rem; margin-bottom: 2rem; display: flex; align-items: center; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                                    <svg style="width: 24px; height: 24px; margin-right: 0.75rem; color: #22c55e;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                    </svg>
                                    <span style="font-weight: 600;">{msg}</span>
                                </div>
                            })}
                            
                            {move || error.get().map(|err| view! {
                                <div style="background: rgba(254, 242, 242, 0.95); backdrop-filter: blur(10px); border: 1px solid #fca5a5; color: #dc2626; padding: 1rem; border-radius: 0.75rem; margin-bottom: 2rem; display: flex; align-items: center; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);" class="animate-shake">
                                    <svg style="width: 24px; height: 24px; margin-right: 0.75rem; color: #ef4444;" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"></path>
                                    </svg>
                                    <span style="font-weight: 600;">{err}</span>
                                </div>
                            })}
                            
                            // Transfer Form
                            <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 2rem; box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);">
                                <div style="display: flex; align-items: center; margin-bottom: 2rem;">
                                    <div style="display: inline-flex; align-items: center; justify-content: center; width: 48px; height: 48px; background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%); border-radius: 0.75rem; margin-right: 1rem;">
                                        <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1"></path>
                                        </svg>
                                    </div>
                                <div>
                                        <h2 style="font-size: 1.5rem; font-weight: 700; color: #1e293b; margin: 0;">"Transfer Details"</h2>
                                        <p style="color: #64748b; margin: 0; font-size: 0.875rem;">"Fill in the details to complete your transfer"</p>
                                    </div>
                                </div>
                                
                                <div style="display: flex; flex-direction: column; gap: 2rem;">
                                    // From Account Section
                                    <div style="background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%); padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #e2e8f0;">
                                        <h3 style="font-size: 1rem; font-weight: 600; color: #374151; margin-bottom: 1rem; display: flex; align-items: center;">
                                            <svg style="width: 20px; height: 20px; margin-right: 0.5rem; color: #3b82f6;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2m2 4h10a2 2 0 002-2v-6a2 2 0 00-2-2H9a2 2 0 00-2 2v6a2 2 0 002 2zm7-5a2 2 0 11-4 0 2 2 0 014 0z"></path>
                                            </svg>
                                            "From Account"
                                        </h3>
                                    <select 
                                            style="width: 100%; padding: 0.875rem 1rem; border: 2px solid #e5e7eb; border-radius: 0.75rem; font-size: 1rem; background: white; color: #374151; transition: all 0.2s; outline: none;"
                                        on:change=move |ev| {
                                            let value = ev.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>().value().parse::<i32>().unwrap_or(0);
                                            set_from_account_id.set(value);
                                        }
                                    >
                                        <For
                                            each=move || accounts.get()
                                            key=|account| account.id
                                            children=move |account| {
                                                view! {
                                                    <option value={account.id}>
                                                        {format!("{} - {} (${:.2})", 
                                                            account.account_type.to_string().to_uppercase(),
                                                            account.account_number,
                                                            account.balance
                                                        )}
                                                    </option>
                                                }
                                            }
                                        />
                                    </select>
                                </div>
                                
                                    // To Account Section
                                    <div style="background: linear-gradient(135deg, #eff6ff 0%, #dbeafe 100%); padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #bfdbfe;">
                                        <h3 style="font-size: 1rem; font-weight: 600; color: #374151; margin-bottom: 1rem; display: flex; align-items: center;">
                                            <svg style="width: 20px; height: 20px; margin-right: 0.5rem; color: #10b981;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16l-4-4m0 0l4-4m-4 4h18M13 8V6a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2h6a2 2 0 002-2v-2"></path>
                                            </svg>
                                            "To Account"
                                        </h3>
                                    <Input
                                            class="custom-input"
                                        placeholder="Enter destination account number"
                                        value=(to_account_number, set_to_account_number)
                                    />
                                </div>
                                
                                    // Amount and Description Section
                                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem;">
                                        <div style="background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%); padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #f59e0b;">
                                            <h3 style="font-size: 1rem; font-weight: 600; color: #374151; margin-bottom: 1rem; display: flex; align-items: center;">
                                                <svg style="width: 20px; height: 20px; margin-right: 0.5rem; color: #f59e0b;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1"></path>
                                                </svg>
                                                "Amount"
                                            </h3>
                                    <Input
                                                class="custom-input"
                                        placeholder="Enter amount (e.g., 100.50)"
                                        value=(amount, set_amount)
                                    />
                                </div>
                                
                                        <div style="background: linear-gradient(135deg, #f3e8ff 0%, #e9d5ff 100%); padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #a855f7;">
                                            <h3 style="font-size: 1rem; font-weight: 600; color: #374151; margin-bottom: 1rem; display: flex; align-items: center;">
                                                <svg style="width: 20px; height: 20px; margin-right: 0.5rem; color: #a855f7;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"></path>
                                                </svg>
                                                "Description"
                                            </h3>
                                    <Input
                                                class="custom-input"
                                                placeholder="Enter description (optional)"
                                        value=(description, set_description)
                                    />
                                        </div>
                                </div>
                                
                                    // Transfer Button
                                    <div style="background: linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%); border-radius: 0.75rem; box-shadow: 0 20px 25px -5px rgba(79, 70, 229, 0.3), 0 10px 10px -5px rgba(79, 70, 229, 0.04);">
                                <Button 
                                            class="custom-button"
                                    loading=loading
                                    on_click=on_transfer
                                >
                                            {move || if loading.get() {
                                                view! {
                                                    <span style="display: flex; align-items: center; justify-content: center;">
                                                        <svg style="width: 20px; height: 20px; margin-right: 0.5rem;" class="animate-spin" fill="none" viewBox="0 0 24 24">
                                                            <circle style="opacity: 0.25;" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                                            <path style="opacity: 0.75;" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                                        </svg>
                                                        "Processing Transfer..."
                                                    </span>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <span style="display: flex; align-items: center; justify-content: center;">
                                                        <svg style="width: 20px; height: 20px; margin-right: 0.5rem;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path>
                                                        </svg>
                                                        "Send Transfer"
                                                    </span>
                                                }.into_any()
                                            }}
                                </Button>
                            </div>
                        </div>
                </div>
                            
                            // Security Notice
                            <div style="background: rgba(239, 246, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid #bfdbfe; color: #1e40af; padding: 1.5rem; border-radius: 0.75rem; margin-top: 2rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                                <div style="display: flex; align-items: flex-start;">
                                    <svg style="width: 24px; height: 24px; color: #3b82f6; margin-top: 0.125rem; margin-right: 0.75rem; flex-shrink: 0;" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clip-rule="evenodd"></path>
                                    </svg>
                                    <div style="font-size: 0.875rem;">
                                        <p style="font-weight: 600; margin-bottom: 0.5rem;">"Security Notice:"</p>
                                        <ul style="color: #1d4ed8; margin: 0; padding-left: 1rem;">
                                            <li>"All transfers are encrypted and secure"</li>
                                            <li>"Double-check account numbers before confirming"</li>
                                            <li>"Transaction confirmations will be sent to your email"</li>
                                            <li>"Contact support if you notice any unauthorized activity"</li>
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Admin page component with full functionality
#[component]
fn AdminPage() -> impl IntoView {
    let current_user = expect_context::<ReadSignal<Option<User>>>();
    let navigate = use_navigate();
    let (active_tab, set_active_tab) = signal("users".to_string());
    
    // Redirect if not authenticated or not admin
    Effect::new(move |_| {
        if let Some(user) = current_user.get() {
            if user.role != UserRole::Admin {
                let _ = navigate("/dashboard", Default::default());
            }
        } else {
            let _ = navigate("/", Default::default());
        }
    });

    view! {
        <Layout has_sider=true>
            <LayoutSider class="bg-white shadow-lg">
                <NavigationSidebar />
            </LayoutSider>
            <Layout>
                <LayoutHeader class="bg-white shadow-sm border-b px-6 py-4">
                    <div class="flex justify-between items-center">
                        <h1 class="text-2xl font-semibold text-gray-900">"Admin Panel"</h1>
                        <LogoutButton />
                    </div>
                </LayoutHeader>
                <div class="p-6 bg-gray-50">
                    <div class="mb-6">
                        <div class="flex space-x-4 border-b">
                            <button
                                class=move || format!("px-4 py-2 font-medium text-sm border-b-2 {}",
                                    if active_tab.get() == "users" { "border-blue-500 text-blue-600" } else { "border-transparent text-gray-500 hover:text-gray-700" }
                                )
                                on:click=move |_| set_active_tab.set("users".to_string())
                            >
                                "User Management"
                            </button>
                            <button
                                class=move || format!("px-4 py-2 font-medium text-sm border-b-2 {}",
                                    if active_tab.get() == "accounts" { "border-blue-500 text-blue-600" } else { "border-transparent text-gray-500 hover:text-gray-700" }
                                )
                                on:click=move |_| set_active_tab.set("accounts".to_string())
                            >
                                "Account Management"
                            </button>
                            <button
                                class=move || format!("px-4 py-2 font-medium text-sm border-b-2 {}",
                                    if active_tab.get() == "transactions" { "border-blue-500 text-blue-600" } else { "border-transparent text-gray-500 hover:text-gray-700" }
                                )
                                on:click=move |_| set_active_tab.set("transactions".to_string())
                            >
                                "Transaction Monitoring"
                            </button>
                        </div>
                    </div>
                    
                    {move || match active_tab.get().as_str() {
                        "users" => view! { <AdminUserManagement /> }.into_any(),
                        "accounts" => view! { <AdminAccountManagement /> }.into_any(),
                        "transactions" => view! { <AdminTransactionMonitoring /> }.into_any(),
                        _ => view! { <AdminUserManagement /> }.into_any(),
                    }}
                </div>
            </Layout>
        </Layout>
    }
}

/// Admin user management component
#[component]
fn AdminUserManagement() -> impl IntoView {
    let (users, set_users) = signal(Vec::<User>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);

    // Load all users
    Effect::new(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            match get_users().await {
                Ok(all_users) => {
                    set_users.set(all_users);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load users: {}", e)));
                }
            }
            set_loading.set(false);
        });
    });

    view! {
        <Card>
            <div class="p-6">
                <div class="flex justify-between items-center mb-4">
                    <h3 class="text-lg font-semibold text-gray-900">"User Management"</h3>
                </div>
                
                {move || error.get().map(|err| view! {
                    <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-4">
                        {err}
                    </div>
                })}
                
                {move || {
                    if loading.get() {
                        view! {
                            <div class="text-center py-8">
                                <p>"Loading users..."</p>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <Table>
                                <TableHeader>
                                    <TableRow>
                                        <TableHeaderCell>"Name"</TableHeaderCell>
                                        <TableHeaderCell>"Email"</TableHeaderCell>
                                        <TableHeaderCell>"Role"</TableHeaderCell>
                                        <TableHeaderCell>"Status"</TableHeaderCell>
                                        <TableHeaderCell>"Created"</TableHeaderCell>
                                        <TableHeaderCell>"Actions"</TableHeaderCell>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                    <For
                                        each=move || users.get()
                                        key=|user| user.id
                                        children=move |user| {
                                            view! {
                                                <TableRow>
                                                    <TableCell>{format!("{} {}", user.first_name, user.last_name)}</TableCell>
                                                    <TableCell>{user.email.clone()}</TableCell>
                                                    <TableCell>
                                                        <Badge>
                                                            {user.role.to_string().to_uppercase()}
                                                        </Badge>
                                                    </TableCell>
                                                    <TableCell>
                                                        <Badge>
                                                            {if user.is_active { "ACTIVE" } else { "INACTIVE" }}
                                                        </Badge>
                                                    </TableCell>
                                                    <TableCell>
                                                        {user.created_at.format("%Y-%m-%d").to_string()}
                                                    </TableCell>
                                                    <TableCell>
                                                        <div class="flex space-x-2">
                                                            <Button size=ButtonSize::Small>"Edit"</Button>
                                                            {if user.is_active {
                                                                view! {
                                                                    <Button size=ButtonSize::Small>"Deactivate"</Button>
                                                                }.into_any()
                                                            } else {
                                                                view! {
                                                                    <Button size=ButtonSize::Small>"Activate"</Button>
                                                                }.into_any()
                                                            }}
                                                        </div>
                                                    </TableCell>
                                                </TableRow>
                                            }
                                        }
                                    />
                                </TableBody>
                            </Table>
                        }.into_any()
                    }
                }}
            </div>
        </Card>
    }
}

/// Admin account management component
#[component]
fn AdminAccountManagement() -> impl IntoView {
    let (accounts, set_accounts) = signal(Vec::<Account>::new());
    let (users, set_users) = signal(Vec::<User>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);
    let (success, set_success) = signal(Option::<String>::None);
    
    // Form fields for creating new account
    let (selected_user_id, set_selected_user_id) = signal(0i32);
    let (account_type, set_account_type) = signal("checking".to_string());
    let (initial_balance, set_initial_balance) = signal(String::from("0.00"));
    let (creating, set_creating) = signal(false);

    // Load accounts and users
    Effect::new(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            
            // Load accounts
            match get_accounts().await {
                Ok(all_accounts) => {
                    set_accounts.set(all_accounts);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load accounts: {}", e)));
                }
            }
            
            // Load users
            match get_users().await {
                Ok(all_users) => {
                    set_users.set(all_users.clone());
                    if let Some(first_user) = all_users.first() {
                        set_selected_user_id.set(first_user.id);
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load users: {}", e)));
                }
            }
            
            set_loading.set(false);
        });
    });

    let create_account_action = Action::new(move |_: &()| {
        let user_id = selected_user_id.get();
        let acc_type = account_type.get();
        let balance_str = initial_balance.get();
        
        async move {
            set_creating.set(true);
            set_error.set(None);
            set_success.set(None);
            
            let balance = match balance_str.parse::<rust_decimal::Decimal>() {
                Ok(bal) => bal,
                Err(_) => {
                    set_error.set(Some("Invalid balance format".to_string()));
                    set_creating.set(false);
                    return;
                }
            };
            
            let account_type_enum = match acc_type.as_str() {
                "checking" => AccountType::Checking,
                "savings" => AccountType::Savings,
                "business" => AccountType::Business,
                _ => AccountType::Checking,
            };
            
            let request = CreateAccountRequest {
                user_id,
                account_type: account_type_enum,
                initial_balance: balance,
            };
            
            match create_account(request).await {
                Ok(_) => {
                    set_success.set(Some("Account created successfully!".to_string()));
                    set_initial_balance.set("0.00".to_string());
                    
                    // Reload accounts
                    match get_accounts().await {
                        Ok(all_accounts) => {
                            set_accounts.set(all_accounts);
                        }
                        Err(_) => {}
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to create account: {}", e)));
                }
            }
            set_creating.set(false);
        }
    });

    let on_create_account = move |_| {
        create_account_action.dispatch(());
    };

    view! {
        <div class="space-y-6">
            <Card>
                <div class="p-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-4">"Create New Account"</h3>
                    
                    {move || success.get().map(|msg| view! {
                        <div class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded mb-4">
                            {msg}
                        </div>
                    })}
                    
                    {move || error.get().map(|err| view! {
                        <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-4">
                            {err}
                        </div>
                    })}
                    
                    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-1">"User"</label>
                            <select 
                                class="w-full p-2 border border-gray-300 rounded-md"
                                on:change=move |ev| {
                                    let value = ev.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>().value().parse::<i32>().unwrap_or(0);
                                    set_selected_user_id.set(value);
                                }
                            >
                                <For
                                    each=move || users.get()
                                    key=|user| user.id
                                    children=move |user| {
                                        view! {
                                            <option value={user.id}>
                                                {format!("{} {} ({})", user.first_name, user.last_name, user.email)}
                                            </option>
                                        }
                                    }
                                />
                            </select>
                        </div>
                        
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-1">"Account Type"</label>
                            <select 
                                class="w-full p-2 border border-gray-300 rounded-md"
                                on:change=move |ev| {
                                    set_account_type.set(ev.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>().value());
                                }
                            >
                                <option value="checking">"Checking"</option>
                                <option value="savings">"Savings"</option>
                                <option value="business">"Business"</option>
                            </select>
                        </div>
                        
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-1">"Initial Balance"</label>
                            <Input
                                placeholder="0.00"
                                value=(initial_balance, set_initial_balance)
                            />
                        </div>
                        
                        <div class="flex items-end">
                            <Button 
                                class="w-full"
                                loading=creating
                                on_click=on_create_account
                            >
                                "Create Account"
                            </Button>
                        </div>
                    </div>
                </div>
            </Card>
            
            <Card>
                <div class="p-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-4">"All Accounts"</h3>
                    
                    {move || {
                        if loading.get() {
                            view! {
                                <div class="text-center py-8">
                                    <p>"Loading accounts..."</p>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <Table>
                                    <TableHeader>
                                        <TableRow>
                                            <TableHeaderCell>"Account Number"</TableHeaderCell>
                                            <TableHeaderCell>"Owner"</TableHeaderCell>
                                            <TableHeaderCell>"Type"</TableHeaderCell>
                                            <TableHeaderCell>"Balance"</TableHeaderCell>
                                            <TableHeaderCell>"Status"</TableHeaderCell>
                                            <TableHeaderCell>"Created"</TableHeaderCell>
                                        </TableRow>
                                    </TableHeader>
                                    <TableBody>
                                        <For
                                            each=move || accounts.get()
                                            key=|account| account.id
                                            children=move |account| {
                                                view! {
                                                    <TableRow>
                                                        <TableCell>{account.account_number.clone()}</TableCell>
                                                        <TableCell>
                                                            {users.get().iter()
                                                                .find(|u| u.id == account.user_id)
                                                                .map(|u| format!("{} {}", u.first_name, u.last_name))
                                                                .unwrap_or_else(|| "Unknown".to_string())
                                                            }
                                                        </TableCell>
                                                        <TableCell>
                                                            <Badge>
                                                                {account.account_type.to_string().to_uppercase()}
                                                            </Badge>
                                                        </TableCell>
                                                        <TableCell>
                                                            <span class="font-mono">
                                                                {"$"}{format!("{:.2}", account.balance)}
                                                            </span>
                                                        </TableCell>
                                                        <TableCell>
                                                            <Badge>
                                                                {if account.is_active { "ACTIVE" } else { "INACTIVE" }}
                                                            </Badge>
                                                        </TableCell>
                                                        <TableCell>
                                                            {account.created_at.format("%Y-%m-%d").to_string()}
                                                        </TableCell>
                                                    </TableRow>
                                                }
                                            }
                                        />
                                    </TableBody>
                                </Table>
                            }.into_any()
                        }
                    }}
                </div>
            </Card>
        </div>
    }
}

/// Admin transaction monitoring component
#[component]
fn AdminTransactionMonitoring() -> impl IntoView {
    let (transactions, set_transactions) = signal(Vec::<Transaction>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);

    // Load all transactions
    Effect::new(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            match get_transactions().await {
                Ok(all_transactions) => {
                    set_transactions.set(all_transactions);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load transactions: {}", e)));
                }
            }
            set_loading.set(false);
        });
    });

    let total_volume = move || {
        transactions.get().iter()
            .map(|txn| txn.amount)
            .fold(rust_decimal::Decimal::ZERO, |sum, amount| sum + amount)
    };

    let pending_count = move || {
        transactions.get().iter()
            .filter(|txn| txn.status == TransactionStatus::Pending)
            .count()
    };

    view! {
        <div class="space-y-6">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <Card>
                    <div class="p-6">
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="text-sm font-medium text-gray-600">"Total Transactions"</p>
                                <p class="text-3xl font-bold text-gray-900">
                                    {move || transactions.get().len()}
                                </p>
                            </div>
                        </div>
                    </div>
                </Card>
                
                <Card>
                    <div class="p-6">
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="text-sm font-medium text-gray-600">"Total Volume"</p>
                                <p class="text-3xl font-bold text-gray-900">
                                    "$" {move || format!("{:.2}", total_volume())}
                                </p>
                            </div>
                        </div>
                    </div>
                </Card>
                
                <Card>
                    <div class="p-6">
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="text-sm font-medium text-gray-600">"Pending Transactions"</p>
                                <p class="text-3xl font-bold text-orange-600">
                                    {move || pending_count()}
                                </p>
                            </div>
                        </div>
                    </div>
                </Card>
            </div>
            
            <Card>
                <div class="p-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-4">"Recent Transactions"</h3>
                    
                    {move || error.get().map(|err| view! {
                        <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-4">
                            {err}
                        </div>
                    })}
                    
                    {move || {
                        if loading.get() {
                            view! {
                                <div class="text-center py-8">
                                    <p>"Loading transactions..."</p>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <Table>
                                    <TableHeader>
                                        <TableRow>
                                            <TableHeaderCell>"Reference"</TableHeaderCell>
                                            <TableHeaderCell>"Type"</TableHeaderCell>
                                            <TableHeaderCell>"Amount"</TableHeaderCell>
                                            <TableHeaderCell>"From Account"</TableHeaderCell>
                                            <TableHeaderCell>"To Account"</TableHeaderCell>
                                            <TableHeaderCell>"Status"</TableHeaderCell>
                                            <TableHeaderCell>"Date"</TableHeaderCell>
                                        </TableRow>
                                    </TableHeader>
                                    <TableBody>
                                        <For
                                            each=move || transactions.get()
                                            key=|txn| txn.id
                                            children=move |txn| {
                                                view! {
                                                    <TableRow>
                                                        <TableCell>
                                                            <span class="font-mono text-sm">
                                                                {txn.reference_number.clone()}
                                                            </span>
                                                        </TableCell>
                                                        <TableCell>
                                                            <Badge>
                                                                {txn.transaction_type.to_string().to_uppercase()}
                                                            </Badge>
                                                        </TableCell>
                                                        <TableCell>
                                                            <span class="font-mono">
                                                                {"$"}{format!("{:.2}", txn.amount)}
                                                            </span>
                                                        </TableCell>
                                                        <TableCell>
                                                            {txn.from_account_id.map(|id| id.to_string()).unwrap_or_else(|| "-".to_string())}
                                                        </TableCell>
                                                        <TableCell>
                                                            {txn.to_account_id.map(|id| id.to_string()).unwrap_or_else(|| "-".to_string())}
                                                        </TableCell>
                                                        <TableCell>
                                                            <Badge>
                                                                {txn.status.to_string().to_uppercase()}
                                                            </Badge>
                                                        </TableCell>
                                                        <TableCell>
                                                            {txn.created_at.format("%Y-%m-%d %H:%M").to_string()}
                                                        </TableCell>
                                                    </TableRow>
                                                }
                                            }
                                        />
                                    </TableBody>
                                </Table>
                            }.into_any()
                        }
                    }}
                </div>
            </Card>
        </div>
    }
} 

/// Deposit page component with payment form
#[component]
fn DepositPage() -> impl IntoView {
    let current_user = expect_context::<ReadSignal<Option<User>>>();
    let auth_token = expect_context::<ReadSignal<Option<String>>>();
    let (accounts, set_accounts) = signal(Vec::<Account>::new());
    let (selected_account_id, set_selected_account_id) = signal(0i32);
    let (deposit_amount, set_deposit_amount) = signal(String::new());
    let (card_holder_name, set_card_holder_name) = signal(String::new());
    let (card_number, set_card_number) = signal(String::new());
    let (expiration_date, set_expiration_date) = signal(String::new());
    let (cvv, set_cvv) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (success, set_success) = signal(Option::<String>::None);
    let (error, set_error) = signal(Option::<String>::None);
    let navigate = use_navigate();
    
    // Redirect if not authenticated
    Effect::new(move |_| {
        if auth_token.get().is_none() {
            let _ = navigate("/", Default::default());
        }
    });

    // Load user accounts
    Effect::new(move |_| {
        if let Some(user) = current_user.get() {
            spawn_local(async move {
                match get_accounts_by_user(user.id).await {
                    Ok(user_accounts) => {
                        set_accounts.set(user_accounts.clone());
                        if let Some(first_account) = user_accounts.first() {
                            set_selected_account_id.set(first_account.id);
                        }
                    }
                    Err(e) => {
                        leptos::logging::log!("Failed to load accounts: {}", e);
                    }
                }
            });
        }
    });

    let deposit_action = Action::new(move |_: &()| {
        let account_id = selected_account_id.get();
        let amount_str = deposit_amount.get();
        let holder_name = card_holder_name.get();
        let card_num = card_number.get();
        let exp_date = expiration_date.get();
        let cvv_code = cvv.get();
        
        async move {
            set_loading.set(true);
            set_error.set(None);
            set_success.set(None);
            
            // Basic validation
            if amount_str.is_empty() {
                set_error.set(Some("Please enter deposit amount".to_string()));
                set_loading.set(false);
                return;
            }
            
            if holder_name.is_empty() {
                set_error.set(Some("Please enter card holder name".to_string()));
                set_loading.set(false);
                return;
            }
            
            if card_num.is_empty() || card_num.len() < 16 {
                set_error.set(Some("Please enter a valid card number (16 digits)".to_string()));
                set_loading.set(false);
                return;
            }
            
            if exp_date.is_empty() {
                set_error.set(Some("Please enter expiration date".to_string()));
                set_loading.set(false);
                return;
            }
            
            if cvv_code.is_empty() || cvv_code.len() < 3 {
                set_error.set(Some("Please enter a valid CVV".to_string()));
                set_loading.set(false);
                return;
            }
            
            // Parse amount
            let amount_decimal = match amount_str.parse::<rust_decimal::Decimal>() {
                Ok(amt) => {
                    if amt <= rust_decimal::Decimal::ZERO {
                        set_error.set(Some("Amount must be greater than zero".to_string()));
                        set_loading.set(false);
                        return;
                    }
                    amt
                },
                Err(_) => {
                    set_error.set(Some("Invalid amount format".to_string()));
                    set_loading.set(false);
                    return;
                }
            };
            
            // Simulate payment processing (in real app, this would integrate with payment gateway)
            
            // Create deposit transaction (would typically come from payment gateway)
            match create_deposit(account_id, amount_decimal).await {
                Ok(_) => {
                    set_success.set(Some(format!("Successfully deposited ${:.2} to your account!", amount_decimal)));
                    set_deposit_amount.set(String::new());
                    set_card_holder_name.set(String::new());
                    set_card_number.set(String::new());
                    set_expiration_date.set(String::new());
                    set_cvv.set(String::new());
                }
                Err(e) => {
                    set_error.set(Some(format!("Deposit failed: {}", e)));
                }
            }
            set_loading.set(false);
        }
    });

    let on_deposit = move |_| {
        deposit_action.dispatch(());
    };

    view! {
        <div style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); position: relative; overflow: hidden;">
            // Background elements
            <div style="position: absolute; inset: 0; overflow: hidden; pointer-events: none;">
                <div style="position: absolute; top: -50px; right: -50px; width: 120px; height: 120px; background: rgba(255, 255, 255, 0.1); border-radius: 50%; filter: blur(20px);" class="animate-blob"></div>
                <div style="position: absolute; bottom: -30px; left: -30px; width: 100px; height: 100px; background: rgba(255, 255, 255, 0.08); border-radius: 50%; filter: blur(25px);" class="animate-blob animation-delay-2000"></div>
                <div style="position: absolute; top: 50%; right: 20px; width: 80px; height: 80px; background: rgba(255, 255, 255, 0.06); border-radius: 50%; filter: blur(30px);" class="animate-blob animation-delay-4000"></div>
            </div>
            
            <div style="display: flex; min-height: 100vh; position: relative; z-index: 10;">
                // Sidebar
                <div style="width: 280px; background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border-right: 1px solid rgba(255, 255, 255, 0.2); box-shadow: 5px 0 25px rgba(0, 0, 0, 0.1);">
                <NavigationSidebar />
                </div>
                
                // Main content
                <div style="flex: 1; display: flex; flex-direction: column;">
                    // Header
                    <div style="background: rgba(255, 255, 255, 0.9); backdrop-filter: blur(10px); border-bottom: 1px solid rgba(255, 255, 255, 0.2); padding: 1.5rem 2rem; box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);">
                        <div style="display: flex; justify-content: space-between; align-items: center;">
                            <div style="display: flex; align-items: center;">
                                <div style="display: inline-flex; align-items: center; justify-content: center; width: 48px; height: 48px; background: linear-gradient(135deg, #16a34a 0%, #15803d 100%); border-radius: 0.75rem; margin-right: 1rem;">
                                    <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
                                    </svg>
                                </div>
                                <div>
                                    <h1 style="font-size: 2rem; font-weight: 700; color: #1e293b; margin: 0; letter-spacing: -0.025em;">"Add Money"</h1>
                                    <p style="font-size: 0.875rem; color: #64748b; margin: 0;">"Deposit money to your account securely"</p>
                                </div>
                            </div>
                            <div style="display: flex; align-items: center; gap: 1rem;">
                                {move || current_user.get().map(|user| view! {
                                    <span style="color: #64748b; font-weight: 500;">"Welcome, " {user.first_name} " " {user.last_name}</span>
                                })}
                        <LogoutButton />
                    </div>
                        </div>
                    </div>
                    
                    // Content area
                    <div style="flex: 1; padding: 2rem; background: transparent;">
                        <div style="max-width: 800px; margin: 0 auto;">
                            // Success/Error Messages
                            {move || success.get().map(|msg| view! {
                                <div style="background: rgba(220, 252, 231, 0.95); backdrop-filter: blur(10px); border: 1px solid #86efac; color: #166534; padding: 1rem; border-radius: 0.75rem; margin-bottom: 2rem; display: flex; align-items: center; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                                    <svg style="width: 24px; height: 24px; margin-right: 0.75rem; color: #22c55e;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                    </svg>
                                    <span style="font-weight: 600;">{msg}</span>
                                </div>
                            })}
                            
                            {move || error.get().map(|err| view! {
                                <div style="background: rgba(254, 242, 242, 0.95); backdrop-filter: blur(10px); border: 1px solid #fca5a5; color: #dc2626; padding: 1rem; border-radius: 0.75rem; margin-bottom: 2rem; display: flex; align-items: center; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);" class="animate-shake">
                                    <svg style="width: 24px; height: 24px; margin-right: 0.75rem; color: #ef4444;" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"></path>
                                    </svg>
                                    <span style="font-weight: 600;">{err}</span>
                                </div>
                            })}
                            
                            // Deposit Form
                            <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 1rem; padding: 2rem; box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);">
                                <div style="display: flex; align-items: center; margin-bottom: 2rem;">
                                    <div style="display: inline-flex; align-items: center; justify-content: center; width: 48px; height: 48px; background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%); border-radius: 0.75rem; margin-right: 1rem;">
                                        <svg style="width: 24px; height: 24px; color: white;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"></path>
                                        </svg>
                                    </div>
                                <div>
                                        <h2 style="font-size: 1.5rem; font-weight: 700; color: #1e293b; margin: 0;">"Deposit Details"</h2>
                                        <p style="color: #64748b; margin: 0; font-size: 0.875rem;">"Enter your deposit amount and payment information"</p>
                                    </div>
                                </div>
                                
                                <div style="display: flex; flex-direction: column; gap: 2rem;">
                                    // Account and Amount Section
                                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem;">
                                        <div style="background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%); padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #e2e8f0;">
                                            <h3 style="font-size: 1rem; font-weight: 600; color: #374151; margin-bottom: 1rem; display: flex; align-items: center;">
                                                <svg style="width: 20px; height: 20px; margin-right: 0.5rem; color: #3b82f6;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"></path>
                                                </svg>
                                                "Deposit To Account"
                                            </h3>
                                        <select 
                                                style="width: 100%; padding: 0.875rem 1rem; border: 2px solid #e5e7eb; border-radius: 0.75rem; font-size: 1rem; background: white; color: #374151; transition: all 0.2s; outline: none;"
                                            on:change=move |ev| {
                                                let value = ev.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>().value().parse::<i32>().unwrap_or(0);
                                                set_selected_account_id.set(value);
                                            }
                                        >
                                            <For
                                                each=move || accounts.get()
                                                key=|account| account.id
                                                children=move |account| {
                                                    view! {
                                                        <option value={account.id}>
                                                            {format!("{} - {} (${:.2})", 
                                                                account.account_type.to_string().to_uppercase(),
                                                                account.account_number,
                                                                account.balance
                                                            )}
                                                        </option>
                                                    }
                                                }
                                            />
                                        </select>
                                    </div>
                                    
                                        <div style="background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%); padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #f59e0b;">
                                            <h3 style="font-size: 1rem; font-weight: 600; color: #374151; margin-bottom: 1rem; display: flex; align-items: center;">
                                                <svg style="width: 20px; height: 20px; margin-right: 0.5rem; color: #f59e0b;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1"></path>
                                                </svg>
                                                "Amount to Deposit"
                                            </h3>
                                        <Input
                                                class="custom-input"
                                            placeholder="Enter amount (e.g., 100.50)"
                                            value=(deposit_amount, set_deposit_amount)
                                        />
                                        </div>
                                </div>
                                
                                    // Payment Information Section
                                    <div style="background: linear-gradient(135deg, #eff6ff 0%, #dbeafe 100%); padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #bfdbfe;">
                                        <h3 style="font-size: 1rem; font-weight: 600; color: #374151; margin-bottom: 1rem; display: flex; align-items: center;">
                                            <svg style="width: 20px; height: 20px; margin-right: 0.5rem; color: #3b82f6;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2m2 4h10a2 2 0 002-2v-6a2 2 0 00-2-2H9a2 2 0 00-2 2v6a2 2 0 002 2zm7-5a2 2 0 11-4 0 2 2 0 014 0z"></path>
                                            </svg>
                                            "Payment Information"
                                        </h3>
                                        
                                        <div style="display: flex; flex-direction: column; gap: 1rem;">
                                        <div>
                                                <label style="display: block; font-size: 0.875rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem;">"Card Holder Name"</label>
                                            <Input
                                                    class="custom-input"
                                                placeholder="Enter card holder name"
                                                value=(card_holder_name, set_card_holder_name)
                                            />
                                        </div>
                                        
                                        <div>
                                                <label style="display: block; font-size: 0.875rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem;">"Card Number"</label>
                                            <Input
                                                    class="custom-input"
                                                placeholder="1234 5678 9012 3456"
                                                value=(card_number, set_card_number)
                                            />
                                        </div>
                                        
                                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
                                            <div>
                                                    <label style="display: block; font-size: 0.875rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem;">"Expiration Date"</label>
                                                <Input
                                                        class="custom-input"
                                                    placeholder="MM/YY"
                                                    value=(expiration_date, set_expiration_date)
                                                />
                                            </div>
                                            
                                            <div>
                                                    <label style="display: block; font-size: 0.875rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem;">"CVV"</label>
                                                <Input
                                                        class="custom-input"
                                                    placeholder="123"
                                                    value=(cvv, set_cvv)
                                                />
                                                </div>
                                        </div>
                                        </div>
                                </div>
                                
                                    // Pay Now Button
                                    <div style="background: linear-gradient(135deg, #16a34a 0%, #15803d 100%); border-radius: 0.75rem; box-shadow: 0 20px 25px -5px rgba(22, 163, 74, 0.3), 0 10px 10px -5px rgba(22, 163, 74, 0.04);">
                                <Button 
                                            class="custom-button"
                                    loading=loading
                                    on_click=on_deposit
                                >
                                            {move || if loading.get() {
                                                view! {
                                                    <span style="display: flex; align-items: center; justify-content: center;">
                                                        <svg style="width: 20px; height: 20px; margin-right: 0.5rem;" class="animate-spin" fill="none" viewBox="0 0 24 24">
                                                            <circle style="opacity: 0.25;" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                                            <path style="opacity: 0.75;" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                                        </svg>
                                                        "Processing Payment..."
                                                    </span>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <span style="display: flex; align-items: center; justify-content: center;">
                                                        <svg style="width: 20px; height: 20px; margin-right: 0.5rem;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2m2 4h10a2 2 0 002-2v-6a2 2 0 00-2-2H9a2 2 0 00-2 2v6a2 2 0 002 2zm7-5a2 2 0 11-4 0 2 2 0 014 0z"></path>
                                                        </svg>
                                                        "Pay Now"
                                                    </span>
                                                }.into_any()
                                            }}
                                </Button>
                            </div>
                        </div>
                </div>
                            
                            // Security Notice
                            <div style="background: rgba(239, 246, 255, 0.95); backdrop-filter: blur(10px); border: 1px solid #bfdbfe; color: #1e40af; padding: 1.5rem; border-radius: 0.75rem; margin-top: 2rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                                <div style="display: flex; align-items: flex-start;">
                                    <svg style="width: 24px; height: 24px; color: #3b82f6; margin-top: 0.125rem; margin-right: 0.75rem; flex-shrink: 0;" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clip-rule="evenodd"></path>
                                    </svg>
                                    <div style="font-size: 0.875rem;">
                                        <p style="font-weight: 600; margin-bottom: 0.5rem;">"Payment Security:"</p>
                                        <ul style="color: #1d4ed8; margin: 0; padding-left: 1rem;">
                                            <li>"All payments are processed securely with 256-bit SSL encryption"</li>
                                            <li>"Your card information is never stored on our servers"</li>
                                            <li>"Deposits typically appear in your account within 1-2 business days"</li>
                                            <li>"Contact support immediately if you notice any unauthorized transactions"</li>
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
} 