use crate::api::*;
use chrono::{DateTime, Utc};
use leptos::ev::toggle;
use leptos::task::spawn_local;
use leptos::{html::table, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use thaw::*;
use web_sys::MouseEvent;
use leptos::ev;

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
    
    view! {
        <h1>"Welcome to Leptos!"</h1>
    }
}

#[derive(Clone, PartialEq)]
pub enum TableState {
    Hidden,
    Angajati,
    Banci,
    Sucursale,
}

#[derive(Clone, PartialEq, Debug)]
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
    let (viewer,set_viewer) = signal(true);
    let (editor,set_editor) = signal(true);
    let on_click_angajati = move |_| {
        set_table_state_editor(TableStateEditor::Hidden);
        set_table_state(TableState::Angajati);
        set_editor(false);
        set_viewer(true);

    };
    let on_click_banci = move |_| {
        set_table_state_editor(TableStateEditor::Hidden);
        set_table_state(TableState::Banci);
        set_editor(false);
        set_viewer(true);
    };
    let on_click_sucursale = move |_| {
        set_table_state_editor(TableStateEditor::Hidden);
        set_table_state(TableState::Sucursale);
        set_editor(false);
        set_viewer(true);
    };

    let on_click_angajati2 = move |_| {
        set_table_state(TableState::Hidden);
        set_table_state_editor(TableStateEditor::Angajati);
        set_viewer(false);
        set_editor(true);
    };
    let on_click_banci2 = move |_| {
        set_table_state(TableState::Hidden);
        set_table_state_editor(TableStateEditor::Banci);
        set_viewer(false);
        set_editor(true);
    };
    let on_click_sucursale2 = move |_| {
        set_table_state(TableState::Hidden);
        set_table_state_editor(TableStateEditor::Sucursale);
        set_viewer(false);
        set_editor(true);
    };


    view! {
            <Layout has_sider=true>
            <LayoutSider attr:style="background-color: #white; padding: 20px;">
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
                <LayoutHeader attr:style="background-color: #white; padding: 20px;">
                    "PIBD PROJECT"
                </LayoutHeader>
                <Layout attr:style="background-color: #white; padding: 20px;">
                    <Show when=viewer>
                    <Tables table_state=table_state.get()/>
                    </Show>
                    <Show when=editor>
                    <Editor table_state_editor=table_state_editor.get()/>
                    </Show>
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
                <NavCategoryItem  slot icon=icondata::AiTableOutlined>
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
                <NavSubItem on:click=on_click_angajati2 value="angajat">
                    "Angajati"
                </NavSubItem>
                <NavSubItem on:click=on_click_banci2 value="banca">
                    "Banci"
                </NavSubItem>
                <NavSubItem on:click=on_click_sucursale2 value="sucursala">
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
                                        <TableHeaderCell>"Banca"</TableHeaderCell>
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
                                                        <TableCell>{sucursala.banca_nume}</TableCell>
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
    </Table>
                            }
                        }
                    }
                }}
            </Transition>
        </div>
    }
}

#[component]
pub fn Editor(#[prop(into)] table_state_editor: Signal<TableStateEditor>)-> impl IntoView {
    let nume = RwSignal::new(String::from(""));
    let prenume = RwSignal::new(String::from(""));
    let banca = RwSignal::new(String::from(""));
    let nume_sucursala = RwSignal::new(String::from(""));
    let input_nume = RwSignal::new(String::from(""));
    let input_prenume = RwSignal::new(String::from(""));
    let input_adresa = RwSignal::new(String::from(""));
    let input_banca = RwSignal::new(String::from(""));
    let current_state = table_state_editor.get();
    leptos::logging::log!("giani{:?}", current_state);
    let submit_banca = move |event| {
        
        let nume_banca_value = input_nume.get().clone();
        let adresa_banca_value = input_adresa.get().clone();
        use crate::api::create_banca;
        spawn_local(async move {
            match create_banca(nume_banca_value, adresa_banca_value).await {
                Ok(banca) => {
                    // Handle successful creation of banca
                    log::info!("Banca creată: {:?}", banca);
                }
                Err(e) => {
                    // Handle error
                    log::error!("Eroare la crearea băncii: {:?}", e);
                }
            }
        });
        leptos::logging::log!("feeedbackkss{:?} si value ala {:?} si babca {:?}",input_nume.get(),input_adresa.get(), input_banca.get());
    };

    let submit_sucursala = move |event| {
        
        let nume_sucursala_value = input_nume.get().clone();
        let adresa_sucursala_value = input_adresa.get().clone();
        
        use crate::api::create_sucursala;
        spawn_local(async move {
            match create_sucursala(nume_sucursala_value, adresa_sucursala_value, input_banca.get()).await {
                Ok(sucursala) => {
                    // Handle successful creation of sucursala
                    log::info!("Sucursala creată: {:?}", sucursala);
                }
                Err(e) => {
                    // Handle error
                    log::error!("Eroare la crearea sucursalei: {:?}", e);
                }
            }
        });
        leptos::logging::log!("feeedbackkss{:?} si value ala {:?} si babca {:?}",nume_sucursala.get(),input_adresa.get(), input_banca.get());
    };

    let submit_angajat = move |event| {
        
        let nume_angajat_value = input_nume.get().clone();
        let prenume_angajat_value = input_prenume.get().clone();
        let telefon_angajat_value = input_adresa.get().clone();

        use crate::api::create_angajat; // Asigură-te că ai o funcție create_angajat în API
        spawn_local(async move {
            match create_angajat(nume_angajat_value, prenume_angajat_value, telefon_angajat_value, input_banca.get()).await {
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
        leptos::logging::log!("feeedbackkss{:?} si value ala {:?} si babca {:?}",nume.get(),prenume.get(), banca.get());
    };
    let banci = OnceResource::new(get_banci());

    let (angajat, set_angajat) = signal(false);
    let (bancaview, set_banca) = signal(false);
    let (sucursalaview, set_sucursala) = signal(false);

    if table_state_editor.get() == TableStateEditor::Angajati {
        set_angajat(true); 
        set_sucursala(false);
        set_banca(false); 
    }

    if table_state_editor.get() == TableStateEditor::Banci {
        set_banca(true); 
        set_angajat(false); 
        set_sucursala(false);
    }

    if table_state_editor.get() == TableStateEditor::Sucursale {
        set_sucursala(true); 
        set_banca(false); 
        set_angajat(false); 
    }
    
    view! {
        <FieldContextProvider>
            <Show when=angajat>
                    <Field label="Nume" name="nume">
                        <Input value=input_nume rules=vec![InputRule::required(true.into())] />
                    </Field>
                    <Field label="Prenume" name="prenume">
                        <Input value=input_prenume rules=vec![InputRule::required(true.into())] />
                    </Field>
                    <Field label="Banca" name="combobox">
                        <Combobox value=input_banca rules=vec![ComboboxRule::required(true.into())] placeholder="Banca" clearable=true>
                        { 
                            // Map over the banca_data and return the combobox options directly
                            let banca_data = match banci.read().as_ref() {
                                Some(Ok(data)) => data.clone(),  // If it's Ok, clone the Vec<Banca>
                                Some(Err(e)) => {
                                    // Handle the error case, for example log the error or return a default value
                                    log::error!("Error fetching banci: {:?}", e);
                                    Vec::new() // Return an empty Vec<Banca> in case of error
                                }
                                None => {
                                    // Handle the None case, for example log the issue or return a default value
                                    log::error!("No data found for banci");
                                    Vec::new() // Return an empty Vec<Banca> in case of None
                                }
                            };
                            banca_data.iter().map(|banca| {
                                view! {
                                    <ComboboxOption value={banca.id.to_string()} text={banca.nume.clone()} />
                                }
                            }).collect::<Vec<_>>() // Collect the views into a Vec
                        }
                        </Combobox>
                    </Field>
                    <div style="margin-top: 8px">
                        <button on:click=submit_angajat>
                            "Submit"
                        </button>
                    </div>
            </Show>
            <Show when=bancaview>
                    <Field label="Nume" name="nume">
                        <Input value=input_nume rules=vec![InputRule::required(true.into())] />
                    </Field>
                    <Field label="Adresa" name="adresa">
                        <Input value=input_adresa rules=vec![InputRule::required(true.into())] />
                    </Field>
                    <div style="margin-top: 8px">
                        <button on:click=submit_banca>
                            "Submit"
                        </button>
                    </div>
            </Show>
            <Show when=sucursalaview>
                    <Field label="Nume" name="adresa">
                        <Input value=input_nume rules=vec![InputRule::required(true.into())] />
                    </Field>
                    <Field label="Adresa" name="adresa">
                        <Input value=input_adresa rules=vec![InputRule::required(true.into())] />
                    </Field>
                    <Field label="Banca" name="combobox">
                        <Combobox value=input_banca rules=vec![ComboboxRule::required(true.into())] placeholder="Banca" clearable=true>
                        { 
                            // Map over the banca_data and return the combobox options directly
                            let banca_data = match banci.read().as_ref() {
                                Some(Ok(data)) => data.clone(),  // If it's Ok, clone the Vec<Banca>
                                Some(Err(e)) => {
                                    // Handle the error case, for example log the error or return a default value
                                    log::error!("Error fetching banci: {:?}", e);
                                    Vec::new() // Return an empty Vec<Banca> in case of error
                                }
                                None => {
                                    // Handle the None case, for example log the issue or return a default value
                                    log::error!("No data found for banci");
                                    Vec::new() // Return an empty Vec<Banca> in case of None
                                }
                            };
                            banca_data.iter().map(|banca| {
                                view! {
                                    <ComboboxOption value={banca.id.to_string()} text={banca.nume.clone()} />
                                }
                            }).collect::<Vec<_>>() // Collect the views into a Vec
                        }
                        </Combobox>
                    </Field>
                    <div style="margin-top: 8px">
                        <button on:click=submit_sucursala>
                            "Submit"
                        </button>
                    </div>
            </Show>
            </FieldContextProvider>
        
    }
}


