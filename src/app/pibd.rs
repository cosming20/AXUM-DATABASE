use leptos::{html::table, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos::task::spawn_local;
use chrono::{DateTime, Utc};
use crate::api::*;
use thaw::*;
use crate::db::*;
use leptos::logging::log;
use web_sys::MouseEvent;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <ConfigProvider>
        <ToasterProvider>
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/{{project-name}}.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("/table") view=MainPage/>
                </Routes>
            </main>
        </Router>
        </ToasterProvider>
        </ConfigProvider>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    // let on_click = move |_| *count.write() += 1;
    let nume = RwSignal::new(String::from("Marcel"));
    let prenume = RwSignal::new(String::from("Ciolacu"));
    let telefon = RwSignal::new(String::from("0757422485"));
    let data_angajarii = RwSignal::new(Utc::now());
    let banca_id = RwSignal::new(1);
    
    let on_click = move |_| {
        let nume_value = nume.get().clone();
        let prenume_value = prenume.get().clone();
        let telefon_value = telefon.get().clone();
        let banca_id_value = banca_id.get();
        use crate::api::create_angajati;
        spawn_local(async move {
            match create_angajati(nume_value, prenume_value, telefon_value, banca_id_value).await {
                Ok(angajat) => {
                    // Handle successful creation of angajat
                    log::info!("Angajat creat: {:?}", angajat);
                }
                Err(e) => {
                    // Handle error
                    log::error!("Eroare la crearea angajatului: {:?}", e);
                }
            }
        });
    };
    // let on_submit = move |_| {
    //     let nume_value = nume.get().clone();
    //     let prenume_value = prenume.get().clone();
    //     let telefon_value = telefon.get().clone();
    //     let data_angajarii_value = data_angajarii.get();
    //     let banca_id_value = banca_id.get();

    //     spawn_local(async move {
    //         match create_angajati(nume_value, prenume_value, telefon_value, data_angajarii_value, banca_id_value).await {
    //             Ok(angajat) => {
    //                 // Handle successful creation of angajat
    //                 log::info!("Angajat creat: {:?}", angajat);
    //             }
    //             Err(e) => {
    //                 // Handle error
    //                 log::error!("Eroare la crearea angajatului: {:?}", e);
    //             }
    //         }
    //     });
    // };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[derive(Clone)]
pub enum TableState {
    Hidden,
    Angajati,
    Banci,
    Sucursale,
}

#[component]
fn MainPage() -> impl IntoView {

    let (table_state, set_table_state) = signal(TableState::Hidden);
    let on_click_angajati = move |_| {
        set_table_state(TableState::Angajati);
 
    };
    let on_click_banci = move |_| {
        set_table_state(TableState::Banci);
  
    };
    let on_click_sucursale = move |_| {
        set_table_state(TableState::Sucursale);
       
    };
    let async_data = LocalResource::new(move || get_angajati());
    view! {
        <Layout has_sider=true>
        <LayoutSider attr:style="background-color: #0078ff99; padding: 20px;">
            <NavComponent
                    on_click_angajati=on_click_angajati
                    on_click_banci=on_click_banci
                    on_click_sucursale=on_click_sucursale
            />
        </LayoutSider>
        <Layout>
            <LayoutHeader attr:style="background-color: #0078ffaa; padding: 20px;">
                "Header"
            </LayoutHeader>
            <Layout attr:style="background-color: #0078ff88; padding: 20px;">
                <Tables table_state=table_state/>
            </Layout>
        </Layout>
    </Layout>
    
}}
#[component]
pub fn NavComponent(
    #[prop()] on_click_angajati: impl Fn(MouseEvent) + 'static + Clone + Copy + Send,
    #[prop()] on_click_banci: impl Fn(MouseEvent) + 'static + Clone + Copy + Send,
    #[prop()] on_click_sucursale: impl Fn(MouseEvent) + 'static + Clone + Copy + Send,
) -> impl IntoView {

    view! {
        <NavDrawer>
            <NavCategory value="table">
                <NavCategoryItem slot icon=icondata::AiTableOutlined>
                    "Table"
                </NavCategoryItem>
                <NavSubItem on:click=on_click_angajati value="target">
                    "Angajati"
                </NavSubItem>
                <NavSubItem on:click=on_click_banci value="above">
                    "Banci"
                </NavSubItem>
                <NavSubItem on:click=on_click_sucursale value="below">
                    "Sucursale"
                </NavSubItem>
            </NavCategory>
            // <NavCategory value="pie">
            //     <NavCategoryItem slot icon=icondata::AiPieChartOutlined>
            //         "Pie Chart"
            //     </NavCategoryItem>
            //     <NavSubItem value="pie-target">
            //         "Pie Target"
            //     </NavSubItem>
            //     <NavSubItem value="pin-above">
            //         "Pin Above"
            //     </NavSubItem>
            //     <NavSubItem value="pin-below">
            //         "Pin Below"
            //     </NavSubItem>
            // </NavCategory>
            <NavItem
                icon=icondata::AiGithubOutlined
                value="github"
                href="https://github.com/cosming20"
                attr:target="_blank"
            >
                "Github"
            </NavItem>
            <NavItem icon=icondata::BiMicrosoftTeams value="teams">
                "Gagea Cosmin"
            </NavItem>
        </NavDrawer>
    }
}

#[component]
pub fn Tables(#[prop(into)] table_state: Signal<TableState>) -> impl IntoView {
    let async_angajati = LocalResource::new(move || get_angajati());
    let async_banci = LocalResource::new(move || get_banci());
    let async_sucursale = LocalResource::new(move || get_sucursale());
    // let angajati_data = async_angajati.read();
    view! {
        <div>
            <Transition fallback=move || {
                view! { <div class="loading">"Loading..."</div> }
            }>
                {move || {
                    match table_state.get() {
                        TableState::Angajati => {
                            view! { 
                                <div>
                                    <p>"Aici va fi tabelul pentru Angajati."</p>
                                </div>
                            }
                        }
                        TableState::Banci => {
                            // Handle the case for Banci
                            view! { 
                                <div>
                                    <p>"Aici va fi tabelul pentru Banci."</p>
                                </div>
                            }
                        }
                        TableState::Sucursale => {
                            // Handle the case for Sucursale
                            view! { 
                                <div>
                                    <p>"Aici va fi tabelul pentru Sucursale."</p>
                                </div>
                            }
                        }
                        TableState::Hidden => {
                            // Handle the case for Hidden
                            view! { 
                                <div>
                                    <p>"Selectați o opțiune pentru a vizualiza datele."</p>
                                </div>
                            }
                        }
                    }
                }}
            </Transition>
        </div>
    }
}

