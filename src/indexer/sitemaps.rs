use std::{io::{prelude::*, BufReader}, sync::{Arc, Mutex}, thread};
use tokio::runtime::Runtime;
use xml::reader::XmlEvent;
use xml::EventReader;
use crate::{debug::gatherers::TimingGatherer, ifcfg, ifncfg, QUEUE_BOT, SITEMAP_BOT};

/// Goes through a sitemap and index content of websites depending on those
/// sitemaps. If a sitemap links to another sitemap, this other sitemap will
/// be analyzed.
/// If a sitemap references other sitemaps, it means that this sitemap is an 
/// index.
pub struct SitemapDefinition {
    pub outgoing_urls: Vec<String>,
    pub is_index: bool
}

impl SitemapDefinition {
    /// Load a sitemap depending on if it's a tarball or an xml document. This
    /// function does not need the `tar_gz_sitemaps` experimental feature to 
    /// be enabled to work. However, it will throw an error if a tarball url is
    /// provided and the feature is disabled.
    pub async fn from_any(url: String) -> Vec<Self> {
        let mut output = vec![];

        if url.ends_with(".xml") {
            output.push(SitemapDefinition::from_xml(url).await);
        } else if url.ends_with(".tar.gz") {
            ifcfg!("tar_gz_sitemaps", {
                let mut sitemaps = SitemapDefinition::from_tar_gz(url).await;
                output.append(&mut sitemaps);
            });
            ifncfg!("tar_gz_sitemaps", {
                println!("Cannot process tarballed sitemaps, missing feature.");
            })
        }
        output
    }

    /// Load a sitemap from an XML document pointed by an url and parse it.
    pub async fn from_xml(url: String) -> Self {
        println!("Reading XML sitemap from {url}");
        SitemapDefinition::parser(surf::get(&url).recv_string().await.unwrap())
    }

    /// Load a sitemap from a compressed archive pointed by an url and parse it.
    /// WARN: This function may fail with too large gzip files as everything is
    /// loded on memory. 
    #[cfg(feature = "tar_gz_sitemaps")]
    pub async fn from_tar_gz(url: String) -> Vec<Self> {
        use tar::Archive;

        let buf = surf::get(&url).await.unwrap().body_bytes().await.unwrap();
        let mut archive = Archive::new(buf.as_slice());

        println!("Reading compressed XML sitemap from {url}");
        archive.entries()
            .unwrap()
            .map(|file| {
                let mut file = file.unwrap();
                let mut content = String::new();

                file.read_to_string(&mut content).unwrap();
                SitemapDefinition::parser(content)
            })
            .collect()
    }

    /// It's the function that actually parses the sitemap.
    fn parser(data: String) -> Self {
        let data = BufReader::new(data.as_bytes());
        let xml_sitemap = EventReader::new(data);
        let mut def = Self { outgoing_urls: vec![], is_index: false };
        let mut nesting = vec![];

        xml_sitemap.into_iter().for_each(|element| match element.unwrap() {
            XmlEvent::StartElement { name, .. } => {
                nesting.push(name);
            }
            XmlEvent::EndElement { name: _ } => {
                nesting.pop();
            }
            XmlEvent::Characters(data) => {
                if data.ends_with(".gz") || data.ends_with(".xml") {
                    def.is_index = true;
                }
                def.outgoing_urls.push(data);
            }
            _ => {}
        });

        def
    }
}

/// The `SitemapBot` will manage sitemaps to visit by creating a queue of every
/// sitemap that shuold be visited on a separate thread. This bot will send 
/// URLs to visit to the `QueueBot`.
pub struct SitemapBot {
    queue: Arc<Mutex<Vec<String>>>
}

unsafe impl Send for SitemapBot {}
unsafe impl Sync for SitemapBot {}
impl SitemapBot {
    pub fn init() -> Self {
        Self { 
            queue: Arc::new(Mutex::new(vec![])) 
        }
    }

    pub fn queue_sitemap(&self, sitemap: String) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(sitemap);
    }

    /// Starts parallel indexing.
    pub fn thread_bot(&self) {
        let queue_clone = self.queue.clone();

        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            let mut tg = { ifcfg!("debug", TimingGatherer::init()) };

            loop {
                let mut queue: Vec<String> = vec![];

                ifcfg!("debug", tg.start_gathering());
                // We free the shared queue and drop it to make it available as
                // soon as possible.
                {
                    let mut guard = queue_clone.lock().unwrap();
                    std::mem::swap(&mut queue, &mut *guard);
                }
                for url in &queue {
                    rt.block_on(async {
                        let sitemap_data = 
                            SitemapDefinition::from_any(url.into()).await;
                        for sitemap in sitemap_data {
                            if !sitemap.is_index {
                                QUEUE_BOT.queue_url(sitemap.outgoing_urls);
                            } else {
                                for sitemap_url in sitemap.outgoing_urls {
                                    SITEMAP_BOT.queue_sitemap(sitemap_url);
                                }
                            }
                        }
                    });
                    ifcfg!("debug", tg.action_done());
                    ifcfg!("debug", {
                        if tg.actions_done % 10 == 0 {
                            tg.log_gathered_data();
                        }
                    });
                }
            }
        });
    }
}
