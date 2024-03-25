# Ask your Documents Anything

Wouldn't it be handy if you could interrogate your documents as though ChatGPT were your personal research assistant?

**Now you can.**

## How it works

The `ayda` command line application is a wrapper around the [AnythingLLM Desktop application](https://useanything.com/). It allows you to interact with your documents in a conversational manner, using the power of ChatGPT to summarise, analyse, and generate content. The model is run locally on your own machine, so you can be sure that your data is secure and private.

The application is designed to work with [Zotero](https://www.zotero.org/), a reference management software. You can import your Zotero library into the application, create workspaces for different collections, and interact with the documents in those workspaces using natural language queries.

The application is designed to be easy to use and flexible, allowing you to focus on your research and writing without getting bogged down in the technical details. The language model can summarise documents, answer questions, generate content, and more, making it a powerful tool for researchers, writers, and students.

## Usage

```bash
> ayda import --source zotero climate
...
539 PDFs imported to workspace 'zotero-climate'

> ayda list

WORKSPACES
folder-logseq
zotero-climate
zotero-covid
zotero-politics

> ayda chat zotero-climate

Prompt:
Summarise Berger 2016 "Interglacials of the last 800,000 years"

Berger (2016) discusses the interglacial periods that have occurred during the
last 800,000 years. He describes these periods as relatively short-lived
intervals between glacial advances, characterized by warmer temperatures and
increased biotic productivity. The most recent interglacial period, known as
the Holocene, began approximately 11,700 years ago and is currently ongoing.
Berger also highlights the importance of Milankovitch cycles in shaping these
climate oscillations, specifically changes in Earth’s orbital parameters and
axial tilt.
```

## Documentation

For documentation on how to use the `ayda` command line application, see the [documentation](https://richardjlyon.github.io/ayda/).

## Update log

1.0 - Initial release

1.1 - Added support for modifying Zotero abstracts and keywords
