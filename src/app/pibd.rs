use crate::api::*;
use leptos::task::spawn_local;
use leptos::prelude::*;
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
                    "View data"
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
                    "Add data"
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

pub fn with_default_class<T>(default_class: &str, class: T) -> String
where
    T: Into<Option<String>>,
{
    match class.into() {
        Some(name) => format!("{} {}", default_class, name),
        None => default_class.to_string(),
    }
}


#[component]
#[allow(non_snake_case)]
pub fn Overlay(#[prop(optional, into)] class: Signal<String>, children: Children) -> impl IntoView {
    let default_class = "overlay";
    let class = Signal::derive(move || with_default_class(default_class, class()));

    view! { <div class=class>{children()}</div> }
}


#[component]
pub fn Tables(#[prop(into)] table_state: Signal<TableState>) -> impl IntoView {
    let (delete_signal, set_delete) = signal(false);
    let angajati_data = Resource::new(move || delete_signal.get(),|_| get_angajati());
    let banci = OnceResource::new(get_banci());
    let sucursala_data = Resource::new(move || delete_signal.get(), |_| get_sucursale());
    
    let (edit_card, set_edit_card) = signal(false);
    
    let edit = move |_| {
        set_edit_card(true);
        set_delete(true); 

    };

    let on_feedback_succes = move |_| {
        set_edit_card.set(false); 
        set_delete(false); 
    };

    let deletea = move |angajat_id: i32| {
        spawn_local(async move {
            let _ = delete_angajat(angajat_id).await;
            set_delete(true);
        });
        set_delete(false);
            
    };

    let deleteb = move |banca_id: i32| {
        spawn_local(async move {
            let _ = delete_banca(banca_id).await;
            set_delete(true);
        });
        set_delete(false);
            
    };

    let deletes = move |sucursala_id: i32| {
        
        spawn_local(async move {
            let _ = delete_sucursala(sucursala_id).await;
            set_delete(true);
        });
        set_delete(false);
            
    };
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
                                        <TableHeaderCell></TableHeaderCell>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                {            {move || {
                                    match angajati_data.get() {
                                        Some(Ok(angajati)) => {
                                            angajati.into_iter().map(|angajat| {
                                                view! {
                                                    <Show when=edit_card>
                                                    <Overlay>
                                                    <Card>
        <CardHeader>
            <Body1>
                <b>"Edit"</b>
            </Body1>
            <CardHeaderDescription slot>
                <Caption1>"Description"</Caption1>
            </CardHeaderDescription>
        </CardHeader>
        <CardPreview>
            <AngajatEditor angajat_id=angajat.id on_feedback_succes=on_feedback_succes/>
        </CardPreview>
        
    </Card>
                                                        
                                                    </Overlay>
                                                    </Show>
                                                    <TableRow>
                                                        <TableCell>{angajat.nume.clone()}</TableCell>
                                                        <TableCell>{angajat.prenume.clone()}</TableCell>
                                                        <TableCell>{angajat.telefon.clone()}</TableCell>
                                                        <TableCell>{angajat.banca_nume}</TableCell>
                                                        <TableCell>
                                                            <ButtonGroup>
                                                                <Button icon=icondata::AiEditOutlined on_click=edit>"Edit"</Button>
                                                                <Button icon=icondata::AiDeleteOutlined on_click={move |_| deletea(angajat.id.clone())}>"Delete"</Button>
                                                            </ButtonGroup>
                                                        </TableCell>
                                                    </TableRow>
                                                }
                                            }).collect::<Vec<_>>()
                                        }
                                        Some(Err(_e)) => vec![view! {
                                            <Show when=edit_card>
                                                    <Overlay>
                                                        <AngajatEditor angajat_id=0 on_feedback_succes=on_feedback_succes/>
                                                    </Overlay>
                                                    </Show>
                                            <TableRow>
                                                <TableCell>"Error: {e}"</TableCell>
                                            </TableRow>
                                        }],
                                        None => vec![view! {
                                            <Show when=edit_card>
                                                    <Overlay>
                                                    <div class="feedback-card">
                                                        <AngajatEditor angajat_id=0 on_feedback_succes=on_feedback_succes/>
                                                        </div>
                                                    </Overlay>
                                                    </Show>
                                            <TableRow>
                                                <TableCell>"Loading..."</TableCell>
                                            </TableRow>
                                        }],
                                    }
                                }}
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
                                                    <Show when=edit_card>
                                                    <Overlay>
                                                    <Card>
        <CardHeader>
            <Body1>
                <b>"Edit"</b>
            </Body1>
            <CardHeaderDescription slot>
                <Caption1>"Description"</Caption1>
            </CardHeaderDescription>
        </CardHeader>
        <CardPreview>
            <BancaEditor banca_id=banca.id on_feedback_succes=on_feedback_succes/>
        </CardPreview>
        
    </Card>
                                                        
                                                    </Overlay>
                                                    </Show>
                                                    <TableRow>
                                                        <TableCell>{banca.nume.clone()}</TableCell>
                                                        <TableCell>{banca.adresa.clone()}</TableCell>

                                                    </TableRow>
                                                }
                                            }).collect::<Vec<_>>(),
                    
                                            Some(Err(e)) => vec![view! {
                                                <Show when=edit_card>
                                                    <Overlay>
                                                    <div class="feedback-card">
                                                        <AngajatEditor angajat_id=0 on_feedback_succes=on_feedback_succes/>
                                                        </div>
                                                    </Overlay>
                                                    </Show>
                                                <TableRow>
                                                    <TableCell >
                                                        {format!("Error: {}", e)}
                                                    </TableCell>
                                                </TableRow>
                                            }],
                    
                                            None => vec![view! {
                                                <Show when=edit_card>
                                                    <Overlay>
                                                    <div class="feedback-card">
                                                        <AngajatEditor angajat_id=0 on_feedback_succes=on_feedback_succes/>
                                                        </div>
                                                    </Overlay>
                                                    </Show>
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
                                        <TableHeaderCell></TableHeaderCell>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                {
                                    move || {
                                        match sucursala_data.get() {
                                            Some(Ok(sucursale)) => sucursale.clone().into_iter().map(|sucursala| {
                                                view! {
                                                    <Show when=edit_card>
                                                    <Overlay>
                                                    <Card>
        <CardHeader>
            <Body1>
                <b>"Edit"</b>
            </Body1>
            <CardHeaderDescription slot>
                <Caption1>"Description"</Caption1>
            </CardHeaderDescription>
        </CardHeader>
        <CardPreview>
            <SucursalaEditor sucursala_id=sucursala.id on_feedback_succes=on_feedback_succes/>
        </CardPreview>
        
    </Card>
                                                        
                                                    </Overlay>
                                                    </Show>
                                                    <TableRow>
                                                        <TableCell>{sucursala.nume.clone()}</TableCell>
                                                        <TableCell>{sucursala.adresa.clone()}</TableCell>
                                                        <TableCell>{sucursala.banca_nume}</TableCell>
                                                        <TableCell>
                                                            <ButtonGroup>
                                                                <Button icon=icondata::AiEditOutlined on_click=edit>"Edit"</Button>
                                                                <Button icon=icondata::AiDeleteOutlined on_click={move |_| deletes(sucursala.id.clone())}>"Delete"</Button>
                                                            </ButtonGroup>
                                                        </TableCell>
                                                    </TableRow>
                                                }
                                            }).collect::<Vec<_>>(),
                    
                                            Some(Err(e)) => vec![view! {
                                                <Show when=edit_card>
                                                    <Overlay>
                                                    <div class="feedback-card">
                                                        <SucursalaEditor sucursala_id=0 on_feedback_succes=on_feedback_succes/>
                                                        </div>
                                                    </Overlay>
                                                    </Show>
                                                <TableRow>
                                                    <TableCell >
                                                        {format!("Error: {}", e)}
                                                    </TableCell>
                                                </TableRow>
                                            }],
                    
                                            None => vec![view! {
                                                <Show when=edit_card>
                                                <Overlay>
                                                <div class="feedback-card">
                                                    <SucursalaEditor sucursala_id=0 on_feedback_succes=on_feedback_succes/>
                                                    </div>
                                                </Overlay>
                                                </Show>
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
    let submit_banca = move |_event| {
        
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

    let submit_sucursala = move |_event| {
        
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
    };

    let submit_angajat = move |_event| {
        
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
                    <Field label="Telefon" name="telefon">
                        <Input value=input_adresa />
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


#[component]
pub fn AngajatEditor(#[prop(into)] angajat_id: i32, #[prop()] on_feedback_succes: impl Fn(MouseEvent) + 'static + Clone + Copy + Send,)-> impl IntoView{
    let nume = RwSignal::new(String::from(""));
    let prenume = RwSignal::new(String::from(""));
    let banca = RwSignal::new(String::from(""));
    let nume_sucursala = RwSignal::new(String::from(""));
    let input_nume = RwSignal::new(String::from(""));
    let input_prenume = RwSignal::new(String::from(""));
    let input_adresa = RwSignal::new(String::from(""));
    let input_banca = RwSignal::new(String::from(""));
    let submit_angajat = move |event| {

        // use crate::api::create_angajat; // Asigură-te că ai o funcție create_angajat în API
        spawn_local(async move {
            edit_angajat(angajat_id, Some(input_nume.get()), Some(input_prenume.get()), Some(input_adresa.get()), Some(input_banca.get())).await;
            on_feedback_succes(event);
        });
    };
    let banci = OnceResource::new(get_banci());

                                    view!{
                                        <FieldContextProvider>
                    <Field label="Nume" name="nume">
                        <Input value=input_nume rules=vec![InputRule::required(true.into())] />
                    </Field>
                    <Field label="Prenume" name="prenume">
                        <Input value=input_prenume rules=vec![InputRule::required(true.into())] />
                    </Field>
                    <Field label="Telefon" name="telefon">
                        <Input value=input_adresa />
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
            </FieldContextProvider>
}
}

#[component]
pub fn SucursalaEditor(#[prop(into)] sucursala_id: i32, #[prop()] on_feedback_succes: impl Fn(MouseEvent) + 'static + Clone + Copy + Send,)-> impl IntoView{
    let nume = RwSignal::new(String::from(""));
    let prenume = RwSignal::new(String::from(""));
    let banca = RwSignal::new(String::from(""));
    let input_nume = RwSignal::new(String::from(""));
    let input_adresa = RwSignal::new(String::from(""));
    let input_banca = RwSignal::new(String::from(""));
    let submit_sucursala = move |event| {

        // use crate::api::create_angajat; // Asigură-te că ai o funcție create_angajat în API
        spawn_local(async move {
            let _ =edit_sucursala(sucursala_id, Some(input_nume.get()), Some(input_adresa.get()), Some(input_banca.get())).await;
            on_feedback_succes(event);
        });
    };
    let banci = OnceResource::new(get_banci());

                                    view!{
                                        <FieldContextProvider>
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
            </FieldContextProvider>
}
}

#[component]
pub fn BancaEditor(#[prop(into)] banca_id: i32, #[prop()] on_feedback_succes: impl Fn(MouseEvent) + 'static + Clone + Copy + Send,)-> impl IntoView{

    let input_nume = RwSignal::new(String::from(""));
    let input_adresa = RwSignal::new(String::from(""));

    let modal_ref = NodeRef::new();
    let _ = on_click_outside(modal_ref, move |_| {
        on_feedback_succes(MouseEvent::new("click").unwrap())
    });
    let submit_banca = move |event| {
        spawn_local(async move {
            let _ =edit_banca(banca_id, Some(input_nume.get()), Some(input_adresa.get())).await;
            on_feedback_succes(event);
        });
        
    };

                                    view!{
                                        <div node_ref=modal_ref>
                                        <FieldContextProvider>
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
            </FieldContextProvider>
                                        </div>
}
}
         