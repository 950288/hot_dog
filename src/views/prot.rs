use std::io::{BufReader, Cursor};
use pdbtbx::Format;
use crate::Route;
use dioxus::{logger::tracing, prelude::*};
use pdbtbx::ReadOptions;

const BLOG_CSS: Asset = asset!("/assets/styling/prot.scss");

#[component]
pub fn Prot() -> Element {
    let mut filenames: Signal<Vec<String>> = use_signal(Vec::new);
    let mut files_uploaded: Signal<Vec<String>> = use_signal(Vec::new);

    let mut atoms: Signal<Vec<String>> = use_signal(Vec::new);

    use_effect(move || {
        let files = &files_uploaded.read();
        for file in files.iter() {
            atoms.write().clear();
            let (mut pdb, _errors) = ReadOptions::new().set_format(Format::Pdb).read_raw(BufReader::new(Cursor::new(file))).unwrap();
            // You can loop over all atoms within 3.5 Aͦ of a specific atom
            // Note: The `locate_within_distance` method takes a squared distance
            let tree = pdb.create_atom_rtree();
            for atom in tree {
                // tracing::info!("{}", atom);
                atoms.write().push(atom.to_string());
            }
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS}

        div {
            class: "prot container",
            h1 {
                class: "header",
                "Protein"
            }
            main {
                class: "main col-20 col-md-24 offset-md-0 offset-2",
                h2 {
                    "Protein Function Prediction"
                }
                input {
                    // tell the input to pick a file
                    r#type: "file",
                    // list the accepted extensions
                    accept: ".pdb",
                    // pick multiple files
                    multiple: false,
                    onchange: move |evt| {
                        async move {
                            if let Some(file_engine) = evt.files() {
                                let files = file_engine.files();
                                for file_name in &files {
                                    filenames.write().push(file_name.clone());
                                    if let Some(file) = file_engine.read_file_to_string(file_name).await
                                    {
                                        files_uploaded.write().push(file);
                                    }
                                }
                            }
                        }
                    }
                },
                br {

                },
                div {
                    for atom in atoms() {
                        span {
                           "{atom}"
                        }
                        br {

                        }
                    }
                }
            }

        }
    }
}
