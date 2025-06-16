use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use serde_xml_rs::from_str;


const INDEX_HTML: &str = include_str!("index.html");
const TAG_HTML: &str = include_str!("tag.html");
const SUBPAGE_HTML: &str = include_str!("subpage.html");

const ENTRIES_PER_PAGE:usize = 10;

#[derive(Debug, Deserialize, Clone)]
struct Website {
    url: String,
    title: String,
    description: String,
    tags: String, // Comma-separated tags
    score: u8,    // User-defined score (0-255)
}

impl Website {
    pub fn to_html(&self) -> String {
        format!(
            r#"
<div class="website">
    <a href="{url}" target="_blank">{title}</a>
    <p>{description}</p>
</div>
"#,
            url = self.url,
            title = self.title,
            description = self.description,
        )
    }
}

#[derive(Debug, Deserialize)]
struct List {
    #[serde(rename = "website")]
    websites: Vec<Website>,
}

#[derive(Debug, Deserialize)]
struct Channel {
    title: String,
    description: String,
    list: List,
}


use std::path::Path;
use std::env;
fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() <= 1 {
		println!("No arguments provided. Please provide an XML file as an argument.");
		return Ok(());
	}
	let binding = args[1].to_string();
	let file_path = std::path::Path::new(&binding);

    // Read the XML file content
    let xml_content = fs::read_to_string(file_path).expect("Unable to read file");

    // Parse the XML content
    let channel: Channel = match from_str(&xml_content) {
        Ok(channel) => channel,
        Err(e) => {
            eprintln!("Error parsing XML: {:?}", e);
            return Ok(()); // Exit if parsing fails
        },
    };

    // Sort websites by score in descending order
    let mut websites = channel.list.websites;
    websites.sort_by_key(|website| std::cmp::Reverse(website.score));

    // Generate tag map
    let mut tag_map: HashMap<String, Vec<Website>> = HashMap::new();
    for website in websites.clone() {
        for tag in website.tags.split(',') {
            let tag = tag.trim().to_string(); // Trim whitespace
            tag_map.entry(tag).or_insert_with(Vec::new).push(website.clone());
        }
    }

    // Generate and write index.html
    let mut index_html:String = INDEX_HTML.to_string();
    index_html = index_html.replace("$TITLE", &channel.title);
    index_html = index_html.replace("$DESCRIPTION", &channel.description);
    let mut navcloud = String::new();

    let mut tags: Vec<_> = tag_map.keys().collect();
    tags.sort();
    for tag in tags {
        navcloud += &format!("<a href=\"tags/{}/index.html\">{}</a> ", tag, tag);
    }

    index_html = index_html.replace("$ENTRIES", &navcloud);

    std::fs::create_dir_all("output").expect("Failed to create output directory");
    std::fs::create_dir_all("output/tags").expect("Failed to create output directory");
    let mut index_file = File::create("output/index.html").expect("Unable to create index.html");

    index_file.write_all(index_html.as_bytes()).expect("Unable to write index.html");

    // Generate and write each tag's HTML file
    for (tag, websites) in tag_map {
        generate_tag_html(tag, websites);
    }

    println!("HTML files have been generated.");
    Ok(())
}


fn generate_tag_html(tag: String, websites: Vec<Website>) {
	let output_dir = format!("output/tags/{}", tag);
	std::fs::create_dir_all(&output_dir).expect("Failed to create output directory");
	let number_of_subpages = (websites.len() + ENTRIES_PER_PAGE - 1) / ENTRIES_PER_PAGE;
	let mut tag_html:String = TAG_HTML.to_string();
	let mut navbar = String::new();
	for i in 0..number_of_subpages {
		navbar += &format!("<a href=\"{}.html\" target=\"view\">{}</a>", i, i);
	}
	tag_html = tag_html.replace("$NAVBAR", &navbar);
	tag_html = tag_html.replace("$TAG", &tag);

	let mut tag_file = File::create(format!("{}/index.html", output_dir)).expect("Unable to create tag index.html file");
	tag_file.write_all(tag_html.as_bytes()).expect("Unable to write subpage HTML file");

	let mut i = 0;
	while i < websites.len() {
		let end = usize::min(i + ENTRIES_PER_PAGE, websites.len());
		let chunk = &websites[i..end];
		let subpage_html = generate_subpage((&chunk).to_vec());
		let mut subpage_file = File::create(format!("{}/{}.html", output_dir, i / ENTRIES_PER_PAGE)).expect("Unable to create subpage HTML file");
		subpage_file.write_all(subpage_html.as_bytes()).expect("Unable to write subpage HTML file");
		i += ENTRIES_PER_PAGE;
	}

}


fn generate_subpage(chunk: Vec<Website>) -> String {
	let mut subpage_html:String = SUBPAGE_HTML.to_string();
	let mut entries = String::new();
	for website in chunk {
		entries += &website.to_html();
	}
	subpage_html = subpage_html.replace("$ENTRIES", &entries);
	return subpage_html;
}
