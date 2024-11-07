use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use serde_xml_rs::from_str;

#[derive(Debug, Deserialize, Clone)]
struct Website {
    url: String,
    title: String,
    description: String,
    tags: String, // Comma-separated tags
    score: u8,    // User-defined score (0-255)
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
    list: List, // Use the List struct to hold websites
}

// Generate HTML for a list of websites
fn websites_to_html(sites: Vec<Website>) -> String {
    let mut html = String::new();
    let mut group = 0;
    let entries_per_page = 10;
    for i in 0..sites.len() {
        if i % entries_per_page == 0 {
            // Close the previous group div if it's not the first group
            if group > 0 {
                html.push_str("</div>"); // Close the previous group div
            }
            group += 1; // Increment the group counter
            html.push_str(&format!("<div class=\"group\" id=\"{}\">", group)); // Start a new group div
        }

        // Add the site item
        html.push_str(&format!(
            "<div class=\"item\"><p><a href=\"{}\">{}</a><br>{}<br><i>{}</i></p></div>",
            sites[i].url, sites[i].title, sites[i].description, sites[i].tags
        ));
    }

    // Close the last group div if it was opened
    if group > 0 {
        html.push_str("</div>");
    }
    
    html.push_str("<center><nav>");
    for i in 1..group+1 {
	html.push_str(&format!("<a href=\"#{}\">{}</a> ", i, i));
    }
    html.push_str("</nav></center>");

    html
}

// Generate the main HTML content for index.html
fn generate_index_html(title: &str, websites: Vec<Website>, tag_map: &HashMap<String, Vec<Website>>) -> String {
    let mut html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{
		background-color: #2c2c2c;
		opacity: 1;
		background-image:  linear-gradient(#ffffff 1px, transparent 1px), linear-gradient(to right, #ffffff 1px, #2c2c2c 1px);
		background-size: 20px 20px;
		font-family: monospace;
		font-size: 14px;
        }}
        header {{
            color: #9ad2d8;
            text-align: center;
            font-size: 22px; 
            font-weight: bold;
	    height: 80px;
        }}
	header img {{
		max-width: 100%;
		float: left;
	}}
        .container {{
            display: flex; /* Use flexbox for two columns */
            max-width: 1200px;
	    color:#ffffff;
	    background-color: #2c2c2c;
	    margin: 0 auto;
	    border: 1px solid #ffffff;
	    padding: 10px;
        }}
        .entries {{
            flex: 1; /* Take remaining space */
            padding: 10px;
        }}
        .tags {{
	    max-width: 20%;
            padding: 30px;
            border-left: 1px solid white;
        }}
        .item {{
            border: 1px solid white;
            margin: 10px 0; /* Margin for top and bottom */
            padding: 15px;
        }}
        a {{
            color: #9ad2d8; /* Text color for unvisited links */
            text-decoration: none; /* Remove underline */
        }}
        a:hover {{
            text-decoration: underline;
        }}

	.group {{
		display: none; /* Hide all groups by default */
	}}

	.group:target {{
		display: block; /* Show the targeted group */
	}}

    </style>
</head>
<body>
    <div class="container">
        <div class="entries">
        <header>
		{}
        </header>
        "#,
        title, title
    );


    html.push_str(&websites_to_html(websites));
    html.push_str("</div>"); // Close entries div

    // Sort tags alphabetically (case-insensitive)
    let mut sorted_tags: Vec<String> = tag_map.keys().cloned().collect();
    sorted_tags.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    // Add tags column
    html.push_str("<div class=\"tags\"><header>Tags</header>");
    for tag in sorted_tags {
        html.push_str(&format!("<a href=\"{}.html#1\">[{}]</a> ", tag, tag));
    }
    html.push_str("<p>Powered by webweaver.</p></div>"); // Close tags div
    html.push_str("</div></body></html>"); // Close container and body
    html
}

// Generate HTML for a specific tag's page
fn generate_tag_html(tag: &str, websites: Vec<Website>) -> String {
    let mut html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - Tag</title>
    <style>
        body {{
                background-color: #2c2c2c;
                opacity: 1;
                background-image:  linear-gradient(#ffffff 1px, transparent 1px), linear-gradient(to right, #ffffff 1px, #2c2c2c 1px);
                background-size: 20px 20px;
                font-family: monospace;
		font-size: 14px;
        }}
        header {{
            color: #9ad2d8;
            text-align: center;
            font-size: 22px;
            font-weight: bold;
            height: 80px;
        }}
        header img {{
                max-width: 100%;
                float: left;
        }}
        .container {{
            display: flex; /* Use flexbox for two columns */
            max-width: 1200px;
            color:#ffffff;
            background-color: #2c2c2c;
            margin: 0 auto;
            border: 1px solid #ffffff;
            padding: 10px;
        }}

        .entries {{
            flex: 1; /* Take remaining space */
            padding: 10px;
        }}
        .tags {{
            max-width: 20%;
            padding: 30px;
            border-left: 1px solid white;
        }}
        .item {{
            border: 1px solid white;
            margin: 10px 0; /* Margin for top and bottom */
            padding: 15px;
        }}
        a {{
            color: #9ad2d8; /* Text color for unvisited links */
            text-decoration: none; /* Remove underline */
        }}
        a:hover {{
            text-decoration: underline;
        }}

        .group {{
                display: none; /* Hide all groups by default */
        }}

        .group:target {{
                display: block; /* Show the targeted group */
        }}

    </style>
</head>
<body>
    <div class="container">
        <div class="entries">
        	<header>
            	{}
        	</header>
        "#,
        tag, tag
    );

    html.push_str(&websites_to_html(websites));
    html.push_str("</div></div></body></html>");
    html
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

    // Generate and save index.html
    let index_html = generate_index_html(&channel.title, websites, &tag_map);
    let mut index_file = File::create("index.html").expect("Unable to create index.html");
    index_file.write_all(index_html.as_bytes()).expect("Unable to write index.html");

    // Generate and save each tag's HTML file
    for (tag, websites) in tag_map {
        let tag_html = generate_tag_html(&tag, websites);
        let mut tag_file = File::create(format!("{}.html", tag)).expect("Unable to create tag HTML file");
        tag_file.write_all(tag_html.as_bytes()).expect("Unable to write tag HTML file");
    }

    println!("HTML files have been generated.");
    Ok(())
}
