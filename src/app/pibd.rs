use crate::api::*;
use chrono::{DateTime, Utc};
use leptos::task::spawn_local;
use leptos::{html::table, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use thaw::*;
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

#[derive(Clone)]
pub enum TableStateEditor {
    Hidden,
    Angajati,
    Banci,
    Sucursale,
}

#[component]
fn MainPage() -> impl IntoView {
    let (table_state, set_table_state) = signal(TableState::Hidden);
    let (table_state_editor, set_table_state_editor) = signal(TableStateEditor::Hidden);

    let on_click_angajati = move |_| {
        set_table_state(TableState::Angajati);
    };
    let on_click_banci = move |_| {
        set_table_state(TableState::Banci);
    };
    let on_click_sucursale = move |_| {
        set_table_state(TableState::Sucursale);
    };

    let on_click_angajati2 = move |_| {
        set_table_state_editor(TableStateEditor::Angajati);
    };
    let on_click_banci2 = move |_| {
        set_table_state_editor(TableStateEditor::Banci);
    };
    let on_click_sucursale2 = move |_| {
        set_table_state_editor(TableStateEditor::Sucursale);
    };
    view! {
            <Layout has_sider=true>
            <LayoutSider attr:style="background-color: #0078ff99; padding: 20px;">
                <NavComponent
                        on_click_angajati=on_click_angajati
                        on_click_banci=on_click_banci
                        on_click_sucursale=on_click_sucursale
                        on_click_angajati2=on_click_angajati2
                        on_click_banci2=on_click_banci2
                        on_click_sucursale2=on_click_sucursale2
                />
            </LayoutSider>
            <Layout>
                <LayoutHeader attr:style="background-color: #0078ffaa; padding: 20px;">
                    "PIBD PROJECT"
                </LayoutHeader>
                <Layout attr:style="background-color: #0078ff88; padding: 20px;">
                    <Tables table_state=table_state/>
                </Layout>
            </Layout>
        </Layout>

    }
}
#[component]
pub fn NavComponent(
    #[prop()] on_click_angajati: impl Fn(MouseEvent) + 'static + Clone + Copy + Send,
    #[prop()] on_click_banci: impl Fn(MouseEvent) + 'static + Clone + Copy + Send,
    #[prop()] on_click_sucursale: impl Fn(MouseEvent) + 'static + Clone + Copy + Send,
    #[prop()] on_click_angajati2: impl Fn(MouseEvent) + 'static + Clone + Copy + Send,
    #[prop()] on_click_banci2: impl Fn(MouseEvent) + 'static + Clone + Copy + Send,
    #[prop()] on_click_sucursale2: impl Fn(MouseEvent) + 'static + Clone + Copy + Send,
) -> impl IntoView {
    view! {
        <NavDrawer>
            <NavCategory value="table">
                <NavCategoryItem slot icon=icondata::AiTableOutlined>
                    "Table Viewer"
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
            <NavCategory value="editor">
                <NavCategoryItem slot icon=icondata::BiCalendarEditRegular>
                    "Table Editor"
                </NavCategoryItem>
                <NavSubItem on:click=on_click_angajati2 value="target">
                    "Angajati"
                </NavSubItem>
                <NavSubItem on:click=on_click_banci2 value="above">
                    "Banci"
                </NavSubItem>
                <NavSubItem on:click=on_click_sucursale2 value="below">
                    "Sucursale"
                </NavSubItem>
            </NavCategory>
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
    let once = OnceResource::new(get_angajati());
    let banci = OnceResource::new(get_banci());
    let sucursale = OnceResource::new(get_sucursale());
    
    view! {
        <div>
            <Transition fallback=move || {
                view! { <div class="loading">"Loading..."</div> }
            }>
                {move || {
                    match table_state.get() {
                        TableState::Angajati => {
                            view! {
                                <Table>
                                <TableHeader>
                                    <TableRow>
                                        <TableHeaderCell>"Nume"</TableHeaderCell>
                                        <TableHeaderCell>"Prenume"</TableHeaderCell>
                                        <TableHeaderCell>"Telefon"</TableHeaderCell>
                                        <TableHeaderCell>"Banca ID"</TableHeaderCell>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                {
                                    move || {
                                  
                                        let angajati_data = once.read().as_ref().cloned();
                                        match angajati_data {
                                            Some(Ok(angajati)) => angajati.clone().into_iter().map(|angajat| {
                                                view! {
                                                    <TableRow>
                                                        <TableCell>{angajat.nume.clone()}</TableCell>
                                                        <TableCell>{angajat.prenume.clone()}</TableCell>
                                                        <TableCell>{angajat.telefon.clone()}</TableCell>
                                                        <TableCell>{angajat.banca_id}</TableCell>
                                                    </TableRow>
                                                }
                                            }).collect::<Vec<_>>(),
                    
                                            Some(Err(e)) => vec![view! {
                                                <TableRow>
                                                    <TableCell >
                                                        {format!("Error: {}", e)}
                                                    </TableCell>
                                                </TableRow>
                                            }],
                    
                                            None => vec![view! {
                                                <TableRow>
                                                    <TableCell >
                                                        "Loading..."
                                                    </TableCell>
                                                </TableRow>
                                            }],
                                        }
                                    }
                                }
                                </TableBody>
                            </Table>
                            }
                        }
                        TableState::Banci => {

                            view! {
                                <Table>
                                <TableHeader>
                                    <TableRow>
                                        <TableHeaderCell>"Nume"</TableHeaderCell>
                                        <TableHeaderCell>"Adresa"</TableHeaderCell>
                                        <TableHeaderCell>"Sucursala ID"</TableHeaderCell>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                {
                                    move || {
                                  
                                        let banca_data = banci.read().as_ref().cloned();
                                        match banca_data {
                                            Some(Ok(banci)) => banci.clone().into_iter().map(|banca| {
                                                view! {
                                                    <TableRow>
                                                        <TableCell>{banca.nume.clone()}</TableCell>
                                                        <TableCell>{banca.adresa.clone()}</TableCell>
                                                        <TableCell>{banca.sucursala_id}</TableCell>
                                                    </TableRow>
                                                }
                                            }).collect::<Vec<_>>(),
                    
                                            Some(Err(e)) => vec![view! {
                                                <TableRow>
                                                    <TableCell >
                                                        {format!("Error: {}", e)}
                                                    </TableCell>
                                                </TableRow>
                                            }],
                    
                                            None => vec![view! {
                                                <TableRow>
                                                    <TableCell >
                                                        "Loading..."
                                                    </TableCell>
                                                </TableRow>
                                            }],
                                        }
                                    }
                                }
                                </TableBody>
                            </Table>
                            }
                        }
                        TableState::Sucursale => {
                            
                            view! {
                                <Table>
                                <TableHeader>
                                    <TableRow>
                                        <TableHeaderCell>"Nume"</TableHeaderCell>
                                        <TableHeaderCell>"Adresa"</TableHeaderCell>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                {
                                    move || {
                                  
                                        let sucursala_data = sucursale.read().as_ref().cloned();
                                        match sucursala_data {
                                            Some(Ok(sucursale)) => sucursale.clone().into_iter().map(|sucursala| {
                                                view! {
                                                    <TableRow>
                                                        <TableCell>{sucursala.nume.clone()}</TableCell>
                                                        <TableCell>{sucursala.adresa.clone()}</TableCell>
                                                    </TableRow>
                                                }
                                            }).collect::<Vec<_>>(),
                    
                                            Some(Err(e)) => vec![view! {
                                                <TableRow>
                                                    <TableCell >
                                                        {format!("Error: {}", e)}
                                                    </TableCell>
                                                </TableRow>
                                            }],
                    
                                            None => vec![view! {
                                                <TableRow>
                                                    <TableCell >
                                                        "Loading..."
                                                    </TableCell>
                                                </TableRow>
                                            }],
                                        }
                                    }
                                }
                                </TableBody>
                            </Table>
                            }
                        }
                        TableState::Hidden => {
                            // Handle the case for Hidden
                            view! {
                                <Table>
        <TableHeader>
            <TableRow>
                <TableHeaderCell>"Here you can see the content of the tables"</TableHeaderCell>
            </TableRow>
        </TableHeader>
        // <TableBody>
        //     <TableRow>
        //         <TableCell>
        //             <TableCellLayout>
        //                 "div"
        //             </TableCellLayout>
        //         </TableCell>
        //         <TableCell>
        //             <TableCellLayout>
        //                 "2"
        //             </TableCellLayout>
        //         </TableCell>
        //         <TableCell>
        //             <TableCellLayout>
        //                 "2023-10-08"
        //             </TableCellLayout>
        //         </TableCell>
        //     </TableRow>
        //     <TableRow>
        //         <TableCell>"span"</TableCell>
        //         <TableCell>"2"</TableCell>
        //         <TableCell>"2023-10-08"</TableCell>
        //     </TableRow>
        // </TableBody>
    </Table>
                            }
                        }
                    }
                }}
            </Transition>
        </div>
    }
}
