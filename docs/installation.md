Three steps are required to install the software:

1. Install [LM Studio](https://lmstudio.ai/)
2. Install [AnythingLLM Desktop](https://useanything.com/)
3. Install [ADA command line applicaton](#ada-command-line-application)

For instructions on how to install LM Studio and AnythingLLM Desktop, please refer to this video:

<iframe width="1024" height="600" src="https://www.youtube.com/embed/-Rs8-M-xBFI?si=v-aDwFJisewO2vxo" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

After installation, using the instructions in the video, obtain and make a note of your LM Studio API key, and its IP address and port.

## Zotero configuration

To use the Zotero integration, you need to obtain an API key and user ID from Zotero. You can find instructions on how to do this in your [Zotero account](https://www.zotero.org/settings/keys).

You also need the path to the Zotero data directory on your computer. This is usually located in your home directory under `Zotero/storage`.

## First run

The first time the application is run, it will ask for the following information:

- LM Studio API key
- LM Studio IP address
- LM Studio port
- Zotero API key
- Zotero user ID
- Zotero data directory

These will be saved in a configuration file on your file system. To edit this file later, use the command `ayda config`.
