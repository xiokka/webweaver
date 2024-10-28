# webweaver
## Description
Generate a bookmarks page from an XML file.

The XML structure must be:
```xml
<?xml version="1.0" encoding="UTF-8" ?>
<channel>
        <title>Your Bookmarks Website Title</title>
        <description>Your Bookmarks Website Description</description>
        <list>
          <website>
            <url>https://xiokka.neocities.org/</url>
            <title>Xiokka's Digital Home</title>
            <description>The Best Website On The Entire Universe.</description>
            <tags>Personal, Neocities, Oldweb</tags> Comma-separated tags
            <score>255</score> User-defined scored. Unsigned 8-bit integer (0-255). Defines website priority on the list.
          </website>
        </list>
</channel>
```

## Usage

### Generate website
```bash
webweaver sites.xml
```
