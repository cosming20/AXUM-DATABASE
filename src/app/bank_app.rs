use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos_router::hooks::use_navigate;
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
        <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center p-4">
            <Card class="w-full max-w-md">
                <CardHeader>
                    <div class="text-center">
                        <h1 class="text-3xl font-bold text-gray-900 mb-2">"SecureBank"</h1>
                        <p class="text-gray-600">"Welcome back! Please sign in to your account."</p>
                    </div>
                </CardHeader>
                <div class="space-y-4 p-6">
                    {move || error.get().map(|err| view! {
                        <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
                            {err}
                        </div>
                    })}
                    
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"Email"</label>
                        <Input
                            placeholder="Enter your email"
                            value=(email, set_email)
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"Password"</label>
                        <Input
                            input_type=InputType::Password
                            placeholder="Enter your password"
                            value=(password, set_password)
                        />
                    </div>
                    <Button
                        class="w-full"
                        loading=loading
                        on_click=on_login
                    >
                        "Sign In"
                    </Button>
                    
                    <div class="text-sm text-gray-600 text-center">
                        <p>"Test credentials:"</p>
                        <p>"Email: test@securebank.test"</p>
                        <p>"Password: password123"</p>
                    </div>
                    
                    <div class="text-center">
                        <p class="text-sm text-gray-600">
                            "Don't have an account? "
                            <a href="/signup" class="text-blue-600 hover:text-blue-500 font-medium">"Sign up"</a>
                        </p>
                    </div>
                </div>
            </Card>
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
        <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center p-4">
            <Card class="w-full max-w-md">
                <CardHeader>
                    <div class="text-center">
                        <h1 class="text-3xl font-bold text-gray-900 mb-2">"SecureBank"</h1>
                        <p class="text-gray-600">"Create your account to get started."</p>
                    </div>
                </CardHeader>
                <div class="space-y-4 p-6">
                    {move || {
                        if success.get() {
                            view! {
                                <div class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded text-center">
                                    <p class="font-medium">"Account created successfully!"</p>
                                    <p class="text-sm">"Redirecting to login..."</p>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div>
                                    {move || error.get().map(|err| view! {
                                        <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-4">
                                            {err}
                                        </div>
                                    })}
                                    
                                    <div class="grid grid-cols-2 gap-4">
                                        <div>
                                            <label class="block text-sm font-medium text-gray-700 mb-1">"First Name *"</label>
                                            <Input
                                                placeholder="Enter your first name"
                                                value=(first_name, set_first_name)
                                            />
                                        </div>
                                        <div>
                                            <label class="block text-sm font-medium text-gray-700 mb-1">"Last Name *"</label>
                                            <Input
                                                placeholder="Enter your last name"
                                                value=(last_name, set_last_name)
                                            />
                                        </div>
                                    </div>
                                    
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-1">"Email *"</label>
                                        <Input
                                            input_type=InputType::Email
                                            placeholder="Enter your email"
                                            value=(email, set_email)
                                        />
                                    </div>
                                    
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-1">"Phone"</label>
                                        <Input
                                            input_type=InputType::Tel
                                            placeholder="Enter your phone number"
                                            value=(phone, set_phone)
                                        />
                                    </div>
                                    
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-1">"Password *"</label>
                                        <Input
                                            input_type=InputType::Password
                                            placeholder="Enter your password"
                                            value=(password, set_password)
                                        />
                                    </div>
                                    
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-1">"Confirm Password *"</label>
                                        <Input
                                            input_type=InputType::Password
                                            placeholder="Confirm your password"
                                            value=(confirm_password, set_confirm_password)
                                        />
                                    </div>
                                    
                                    <Button
                                        class="w-full"
                                        loading=loading
                                        on_click=on_signup
                                    >
                                        "Create Account"
                                    </Button>
                                    
                                    <div class="text-center">
                                        <p class="text-sm text-gray-600">
                                            "Already have an account? "
                                            <a href="/" class="text-blue-600 hover:text-blue-500 font-medium">"Sign in"</a>
                                        </p>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </Card>
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
        <Layout has_sider=true>
            <LayoutSider class="bg-white shadow-lg">
                <NavigationSidebar />
            </LayoutSider>
            <Layout>
                <LayoutHeader class="bg-white shadow-sm border-b px-6 py-4">
                    <div class="flex justify-between items-center">
                        <h1 class="text-2xl font-semibold text-gray-900">"Dashboard"</h1>
                        <div class="flex items-center space-x-4">
                            {move || current_user.get().map(|user| view! {
                                <span class="text-gray-600">"Welcome, " {user.first_name} " " {user.last_name}</span>
                            })}
                            <LogoutButton />
                        </div>
                    </div>
                </LayoutHeader>
                <div class="p-6 bg-gray-50">
                    <DashboardContent />
                </div>
            </Layout>
        </Layout>
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

    view! {
        <div class="p-4">
            <div class="mb-8">
                <h2 class="text-xl font-bold text-gray-900">"SecureBank"</h2>
            </div>
            <nav class="space-y-2">
                <Button
                    class="w-full justify-start"
                    on_click={
                        let navigate = navigate.clone();
                        move |_| { let _ = navigate("/dashboard", Default::default()); }
                    }
                >
                    "Dashboard"
                </Button>
                <Button
                    class="w-full justify-start"
                    on_click={
                        let navigate = navigate.clone();
                        move |_| { let _ = navigate("/accounts", Default::default()); }
                    }
                >
                    "Accounts"
                </Button>
                <Button
                    class="w-full justify-start"
                    on_click={
                        let navigate = navigate.clone();
                        move |_| { let _ = navigate("/transactions", Default::default()); }
                    }
                >
                    "Transactions"
                </Button>
                <Button
                    class="w-full justify-start"
                    on_click={
                        let navigate = navigate.clone();
                        move |_| { let _ = navigate("/transfer", Default::default()); }
                    }
                >
                    "Transfer"
                </Button>
                <Button
                    class="w-full justify-start"
                    on_click={
                        let navigate = navigate.clone();
                        move |_| { let _ = navigate("/admin", Default::default()); }
                    }
                >
                    "Admin"
                </Button>
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
        <div class="space-y-6">
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <Card>
                    <div class="p-6">
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="text-sm font-medium text-gray-600">"Total Balance"</p>
                                <p class="text-3xl font-bold text-gray-900">
                                    "$" {move || format!("{:.2}", total_balance())}
                                </p>
                            </div>
                        </div>
                    </div>
                </Card>
                
                <Card>
                    <div class="p-6">
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="text-sm font-medium text-gray-600">"Active Accounts"</p>
                                <p class="text-3xl font-bold text-gray-900">
                                    {move || accounts.get().len()}
                                </p>
                            </div>
                        </div>
                    </div>
                </Card>
                
                <Card>
                    <div class="p-6">
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="text-sm font-medium text-gray-600">"Account Status"</p>
                                <p class="text-lg font-semibold text-green-600">"Active"</p>
                            </div>
                        </div>
                    </div>
                </Card>
            </div>

            <Card>
                <div class="p-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-4">"Recent Accounts"</h3>
                    {move || {
                        if loading.get() {
                            view! { <div>"Loading accounts..."</div> }.into_any()
                        } else if accounts.get().is_empty() {
                            view! { <div>"No accounts found"</div> }.into_any()
                        } else {
                            view! {
                                <div class="space-y-4">
                                    <For
                                        each=move || accounts.get()
                                        key=|account| account.id
                                        children=move |account| {
                                            view! {
                                                <div class="flex justify-between items-center p-4 bg-gray-50 rounded-lg">
                                                    <div>
                                                        <p class="font-medium">{account.account_type.to_string().to_uppercase()} " Account"</p>
                                                        <p class="text-sm text-gray-600">"Account #: " {account.account_number.clone()}</p>
                                                    </div>
                                                    <div class="text-right">
                                                        <p class="font-bold">"$" {format!("{:.2}", account.balance)}</p>
                                                        <p class="text-sm text-gray-600">{account.currency.clone()}</p>
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
            </Card>
        </div>
    }
}

/// Accounts page component with real data
#[component]
fn AccountsPage() -> impl IntoView {
    let current_user = expect_context::<ReadSignal<Option<User>>>();
    let (accounts, set_accounts) = signal(Vec::<Account>::new());
    let (loading, set_loading) = signal(true);
    let navigate = use_navigate();
    
    // Redirect if not authenticated
    Effect::new(move |_| {
        if current_user.get().is_none() {
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

    view! {
        <Layout has_sider=true>
            <LayoutSider class="bg-white shadow-lg">
                <NavigationSidebar />
            </LayoutSider>
            <Layout>
                <LayoutHeader class="bg-white shadow-sm border-b px-6 py-4">
                    <div class="flex justify-between items-center">
                        <h1 class="text-2xl font-semibold text-gray-900">"My Accounts"</h1>
                        <LogoutButton />
                    </div>
                </LayoutHeader>
                <div class="p-6 bg-gray-50">
                    {move || {
                        if loading.get() {
                            view! { <div>"Loading accounts..."</div> }.into_any()
                        } else if accounts.get().is_empty() {
                            view! { <div>"No accounts found"</div> }.into_any()
                        } else {
                            view! {
                                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                    <For
                                        each=move || accounts.get()
                                        key=|account| account.id
                                        children=move |account| {
                                            let status_class = if account.is_active { 
                                                "bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs font-medium"
                                            } else { 
                                                "bg-red-100 text-red-800 px-2 py-1 rounded-full text-xs font-medium"
                                            };
                                            view! {
                                                <Card>
                                                    <div class="p-6">
                                                        <h3 class="text-lg font-semibold text-gray-900 mb-2">
                                                            {account.account_type.to_string().to_uppercase()} " Account"
                                                        </h3>
                                                        <p class="text-gray-600 mb-4">
                                                            "Account #: " {account.account_number.clone()}
                                                        </p>
                                                        <p class="text-2xl font-bold text-gray-900">
                                                            "$" {format!("{:.2}", account.balance)}
                                                        </p>
                                                        <p class="text-sm text-gray-600 mt-1">
                                                            {account.currency.clone()}
                                                        </p>
                                                        <div class="mt-4">
                                                            <span class=status_class>
                                                                {if account.is_active { "Active" } else { "Inactive" }}
                                                            </span>
                                                        </div>
                                                    </div>
                                                </Card>
                                            }
                                        }
                                    />
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </Layout>
        </Layout>
    }
}

/// Transactions page component with real data
#[component]
fn TransactionsPage() -> impl IntoView {
    let current_user = expect_context::<ReadSignal<Option<User>>>();
    let (transactions, set_transactions) = signal(Vec::<Transaction>::new());
    let (loading, set_loading) = signal(true);
    let navigate = use_navigate();
    
    // Redirect if not authenticated
    Effect::new(move |_| {
        if current_user.get().is_none() {
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

    view! {
        <Layout has_sider=true>
            <LayoutSider class="bg-white shadow-lg">
                <NavigationSidebar />
            </LayoutSider>
            <Layout>
                <LayoutHeader class="bg-white shadow-sm border-b px-6 py-4">
                    <div class="flex justify-between items-center">
                        <h1 class="text-2xl font-semibold text-gray-900">"Transaction History"</h1>
                        <LogoutButton />
                    </div>
                </LayoutHeader>
                <div class="p-6 bg-gray-50">
                    <Card>
                        <div class="p-6">
                            {move || {
                                if loading.get() {
                                    view! { <div>"Loading transactions..."</div> }.into_any()
                                } else if transactions.get().is_empty() {
                                    view! { <div>"No transactions found"</div> }.into_any()
                                } else {
                                    view! {
                                        <Table>
                                            <TableHeader>
                                                <TableRow>
                                                    <TableHeaderCell>"Date"</TableHeaderCell>
                                                    <TableHeaderCell>"Type"</TableHeaderCell>
                                                    <TableHeaderCell>"Description"</TableHeaderCell>
                                                    <TableHeaderCell>"Amount"</TableHeaderCell>
                                                    <TableHeaderCell>"Status"</TableHeaderCell>
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
                                                                    {txn.created_at.format("%Y-%m-%d %H:%M").to_string()}
                                                                </TableCell>
                                                                <TableCell>
                                                                    {txn.transaction_type.to_string().to_uppercase()}
                                                                </TableCell>
                                                                <TableCell>
                                                                    {txn.description.unwrap_or_else(|| "No description".to_string())}
                                                                </TableCell>
                                                                <TableCell>
                                                                    "$" {format!("{:.2}", txn.amount)}
                                                                </TableCell>
                                                                <TableCell>
                                                                    <Badge>
                                                                        {txn.status.to_string().to_uppercase()}
                                                                    </Badge>
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
            </Layout>
        </Layout>
    }
}

/// Transfer page component with real functionality
#[component]
fn TransferPage() -> impl IntoView {
    let current_user = expect_context::<ReadSignal<Option<User>>>();
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
        if current_user.get().is_none() {
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
            
            // Parse amount
            let amount_decimal = match amount_str.parse::<rust_decimal::Decimal>() {
                Ok(amt) => amt,
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
        <Layout has_sider=true>
            <LayoutSider class="bg-white shadow-lg">
                <NavigationSidebar />
            </LayoutSider>
            <Layout>
                <LayoutHeader class="bg-white shadow-sm border-b px-6 py-4">
                    <div class="flex justify-between items-center">
                        <h1 class="text-2xl font-semibold text-gray-900">"Transfer Money"</h1>
                        <LogoutButton />
                    </div>
                </LayoutHeader>
                <div class="p-6 bg-gray-50">
                    <Card class="max-w-2xl mx-auto">
                        <div class="space-y-4 p-6">
                            {move || success.get().map(|msg| view! {
                                <div class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded">
                                    {msg}
                                </div>
                            })}
                            
                            {move || error.get().map(|err| view! {
                                <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
                                    {err}
                                </div>
                            })}
                            
                            <div class="space-y-4">
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">"From Account"</label>
                                    <select 
                                        class="w-full p-2 border border-gray-300 rounded-md"
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
                                
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">"To Account Number"</label>
                                    <Input
                                        placeholder="Enter destination account number"
                                        value=(to_account_number, set_to_account_number)
                                    />
                                </div>
                                
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">"Amount"</label>
                                    <Input
                                        placeholder="Enter amount (e.g., 100.50)"
                                        value=(amount, set_amount)
                                    />
                                </div>
                                
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">"Description (Optional)"</label>
                                    <Input
                                        placeholder="Enter description"
                                        value=(description, set_description)
                                    />
                                </div>
                                
                                <Button 
                                    class="w-full"
                                    loading=loading
                                    on_click=on_transfer
                                >
                                    "Transfer Money"
                                </Button>
                            </div>
                        </div>
                    </Card>
                </div>
            </Layout>
        </Layout>
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