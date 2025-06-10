# joogle

**Search engine made out of hopes**

## Roadmap

- [X] Working indexing and searching.
- [ ] Use a faster, less prone to failure and concurrent database.
- [X] Save queued URLs in a file on exit to recover indexing on restart.
- [ ] Indexing improvements
    - [X] Auto-queueing of URLs found on webpages.
        - [X] Verify `href` values ARE links.
        - [X] Transform relative links into absolute links.
        - [X] Understand why the database fails when auto-queuing websites with
              UTF-16 characters.
    - [X] Saving indexed pages description and title.
    - [X] Solve an issue with data not being resetted on re-indexing.
    - [X] Better algorithm for page scoring based on content.
    - [X] Save page's language data.
    - [ ] Scoring based on external links.
    - [X] Use robots.txt and sitemaps, allowing to only submit a domain name and
        a sitemap URL to the indexer and the bot will do everything by itself.
    - [X] Avoid indexing pages that returned a non-succesful HTTP code.
    - [ ] Improve the search console.
    - [ ] Add an option to make certain URLs trigger a queue by scanning useful
        URLs referred to it. Which would allow to not have infinite indexing
        loops with no meaningful content.
- [ ] Search improvements
    - [ ] Implement result pagination instead of the 100's result limit.
    - [ ] Implement localization segregated search.
    - [ ] Implement a better user experience to navigate through search results
        - [ ] Implement page descriptions.
        - [ ] Make the UI better.
- [ ] Quality of Life
    - [ ] Streamline the development and integration processes of SPAs
- [X] Debug
    - [X] Improve the way the `routes!` macro is used with debug routes.
    - [X] Prevent compiling to production with `debug` features enabled.

## Indexing

Sending a JSON-formatted list of URLs at `/index` starts an indexing process
for those URLs.

Indexing happens by giving a website a score for each word it contains, hence:
- A word have a score defined by `n` as the number of occurences of this word.
- A word present in the title has it's score multiplied by 20.
- A word present in the description has it's score multiplied by 8.
- A word present in a `p` or `span` tag has it's score multiplied by 1.
- A word present in a `h1` tag has it's score multiplied by 15.
- A word present in a `h2` tag has it's score multiplied by 10.
- A word present in a `h3` tag has it's score multiplied by 7.
- A word present in a `h4` tag has it's score multiplied by 5.
- A word present in a `h5` tag has it's score multiplied by 3.

Each word found is lowercased before processing, and word scoring for a specific
website is stored in a SQL database such as, for the **TABLE OF WORD X**:

| URL                 | SCORE |
| ------------------- | ----- |
| www.google.com      |   128 |
| 128.0.0.2           |    16 |

A Type-Token Ratio is also calculated and added to a table where data about the
url is stored. It allows to have an idea of the page quality.

This technique is meant to be upgraded as it's not ideal, the next phase is to
use hyperlinks when indexing websites to determine the domain score (which could
play a role in finding the best results for a query)

Indexing will not index pages that didn't returned a succesful 2XX HTTP code.

## Searching

Search queries are sent at `/search`, the `q` parameter contains the query string.
To get the best search results, the query string is decomposed in a list of
words. A "leaderboard" of matching websites is made and the score a website gets
at indexing for a specific word in the query gets added to it's matching score.

The Type-Token Ratio of each page modifies the final score of a page on the 
search results.

The server returns to the client a list of the matching results starting from
the best one.

This technique is not the best because it means that search results accuracy
depends on the query length.

## How to use?

The whole repository must be cloned. To start the whole infrastructure, the
`cargo run` command must be called.

To prepare the infrastructure to run, the `prepare-apps.sh` file must be run
to compile and set up all SPAs.
