# joogle

**Search engine made out of hopes**

## Roadmap

- [X] Working indexing and searching.
- [ ] Indexing improvements
    - [ ] Trustworthy scores with hyperlinks.
    - [ ] Saving indexed pages and titles.
    - [ ] Solve an issue with data not being resetted on re-indexing.
- [ ] Search improvements
    - [ ] Implement result pagination instead of the 100's result limit.

## Indexing

Sending a JSON-formatted list of URLs at `/index` starts an indexing process
for those URLs.

Indexing happens by giving a website a score for each word it contains, hence:
- A word have a score defined by `n` as the number of occurences of this word.
- A word present in the title has it's score multiplied by 10.
- A word present in the description has it's score multiplied by 5.
- A word shorter than 3 characters is not considered.

Each word found is lowercased before processing, and word scoring for a specific
website is stored in a SQL database such as, for the **TABLE OF WORD X**:

| URL            | SCORE |
| -------------- | ----- |
| www.google.com |   128 |
| 128.0.0.2      |    16 |

This technique is meant to be upgraded as it's not ideal, the next phase is to
use hyperlinks when indexing websites to determine the domain score (which could
play a role in finding the best results for a query)

## Searching

Search queries are sent at `/search`, the `q` parameter contains the query string.
To get the best search results, the query string is decomposed in a list of
words. A "leaderboard" of matching websites is made and the score a website gets
at indexing for a specific word in the query gets added to it's matching score.

The server returns to the client a list of the matching results starting from
the best one.

This technique is not the best because it means that search results accuracy
depends on the query length.
